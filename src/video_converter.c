#include <ctype.h>
#include <libavcodec/avcodec.h>
#include <libavformat/avformat.h>
#include <libavutil/imgutils.h>
#include <libavutil/opt.h>
#include <libswscale/swscale.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#include "video_converter.h"

bool ends_with_ignore_case(const char* str, const char* suffix)
{
    size_t str_len = strlen(str);
    size_t suffix_len = strlen(suffix);
    if (suffix_len > str_len) return false;
    str += str_len - suffix_len;
    for (size_t i = 0; i < suffix_len; i++) {
        if (tolower((unsigned char)str[i]) != tolower((unsigned char)suffix[i])) {
            return false;
        }
    }
    return true;
}

int convert_video(const char* input_path, const char* output_path)
{
    AVFormatContext* in_fmt_ctx = NULL;
    AVFormatContext* out_fmt_ctx = NULL;
    AVCodecContext* dec_ctx = NULL;
    AVCodecContext* enc_ctx = NULL;
    AVStream* in_stream = NULL;
    AVStream* out_stream = NULL;
    AVPacket pkt;
    AVFrame* frame = NULL;
    int video_stream_idx = -1;
    int ret;
    // Open input
    if ((ret = avformat_open_input(&in_fmt_ctx, input_path, NULL, NULL)) < 0) goto fail;
    if ((ret = avformat_find_stream_info(in_fmt_ctx, NULL)) < 0) goto fail;
    // Find video stream
    for (unsigned i = 0; i < in_fmt_ctx->nb_streams; i++) {
        if (in_fmt_ctx->streams[i]->codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
            video_stream_idx = i;
            break;
        }
    }
    if (video_stream_idx < 0) goto fail;
    in_stream = in_fmt_ctx->streams[video_stream_idx];
    // Decoder
    const AVCodec* decoder = avcodec_find_decoder(in_stream->codecpar->codec_id);
    dec_ctx = avcodec_alloc_context3(decoder);
    avcodec_parameters_to_context(dec_ctx, in_stream->codecpar);
    if ((ret = avcodec_open2(dec_ctx, decoder, NULL)) < 0) goto fail;
    // Output setup
    avformat_alloc_output_context2(&out_fmt_ctx, NULL, NULL, output_path);
    if (!out_fmt_ctx) goto fail;
    const AVCodec* encoder; 
    bool is_webm_output = ends_with_ignore_case(output_path, ".webm");
    if (is_webm_output) encoder = avcodec_find_encoder(AV_CODEC_ID_VP8);  // or VP9 
    else encoder = avcodec_find_encoder(AV_CODEC_ID_H264); 
    enc_ctx = avcodec_alloc_context3(encoder);
    if (!encoder) goto fail;
    if (!enc_ctx) goto fail;
    enc_ctx->height = dec_ctx->height;
    enc_ctx->width = dec_ctx->width;
    enc_ctx->pix_fmt = AV_PIX_FMT_YUV420P;
    enc_ctx->time_base = (AVRational){1, 25}; 
    enc_ctx->max_b_frames = 0;
    enc_ctx->has_b_frames = 0;
    out_stream = avformat_new_stream(out_fmt_ctx, NULL);
    if (!out_stream) goto fail;
    if ((ret = avcodec_open2(enc_ctx, encoder, NULL)) < 0) goto fail;
    out_stream->time_base = enc_ctx->time_base;
    if ((ret = avcodec_parameters_from_context(out_stream->codecpar, enc_ctx)) < 0) goto fail;
    if (!(out_fmt_ctx->oformat->flags & AVFMT_NOFILE)) {
        if ((ret = avio_open(&out_fmt_ctx->pb, output_path, AVIO_FLAG_WRITE)) < 0) goto fail;
    }
    if ((ret = avformat_write_header(out_fmt_ctx, NULL)) < 0) goto fail;
    // Main loop
    frame = av_frame_alloc();
    pkt.data = NULL;
    pkt.size = 0;
    int64_t frame_index = 0;
    while ((ret = av_read_frame(in_fmt_ctx, &pkt)) >= 0) {
        if (pkt.stream_index != video_stream_idx) {
            av_packet_unref(&pkt);
            continue;
        }
        ret = avcodec_send_packet(dec_ctx, &pkt);
        av_packet_unref(&pkt);
        if (ret < 0) goto fail;
        while ((ret = avcodec_receive_frame(dec_ctx, frame)) >= 0) {
            frame->pts = frame_index++;
            if ((ret = avcodec_send_frame(enc_ctx, frame)) < 0) goto fail;
            while ((ret = avcodec_receive_packet(enc_ctx, &pkt)) >= 0) {
                // Final prep for writing
                pkt.stream_index = out_stream->index;
                pkt.duration = av_rescale_q(1, enc_ctx->time_base, out_stream->time_base);
                av_packet_rescale_ts(&pkt, enc_ctx->time_base, out_stream->time_base);
                if ((ret = av_interleaved_write_frame(out_fmt_ctx, &pkt)) < 0) {
                    char errbuf[AV_ERROR_MAX_STRING_SIZE];
                    av_strerror(ret, errbuf, sizeof(errbuf));
                    fprintf(stderr, "Write failed: %s\n", errbuf);
                    goto fail;
                }
                av_packet_unref(&pkt);
            }
        }
    }
    // Flush encoder
    avcodec_send_frame(enc_ctx, NULL);
    while (avcodec_receive_packet(enc_ctx, &pkt) == 0) {
        pkt.stream_index = out_stream->index;
        pkt.duration = av_rescale_q(1, enc_ctx->time_base, out_stream->time_base);
        av_packet_rescale_ts(&pkt, enc_ctx->time_base, out_stream->time_base);
        av_interleaved_write_frame(out_fmt_ctx, &pkt);
        av_packet_unref(&pkt);
    }
    av_write_trailer(out_fmt_ctx);
    // Cleanup
    avcodec_free_context(&dec_ctx);
    avcodec_free_context(&enc_ctx);
    avformat_close_input(&in_fmt_ctx);
    if (!(out_fmt_ctx->oformat->flags & AVFMT_NOFILE)) avio_closep(&out_fmt_ctx->pb);
    avformat_free_context(out_fmt_ctx);
    av_frame_free(&frame);
    return 0;
fail:
    fprintf(stderr, "Error occurred: %s\n", av_err2str(ret));
    return -1;
}

