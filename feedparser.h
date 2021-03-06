#pragma once
#include <time.h>
#ifdef __cplusplus
extern "C" {
#endif
    struct DO_NOT_USE;
    struct FeedString{
        char* data;
        unsigned long size;
    };
    struct FeedEntry {
        time_t updated;
        time_t published;
        FeedString id;
        FeedString title;
        FeedString link;
        FeedString summary;
    };

    struct FeedInfo {
        unsigned long size;
        FeedEntry* entries;
        time_t updated;
        FeedString title;
        DO_NOT_USE* datav;
        DO_NOT_USE* dataf;
    };
    FeedInfo* feedparser_parse_url(const char*);
    void feedparser_release_feedinfo(FeedInfo**);
#ifdef __cplusplus
}
#endif
