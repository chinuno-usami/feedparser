#pragma once
#include <time.h>
#ifdef __cplusplus
extern "C" {
#endif
    struct Feed;
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
        DO_NOT_USE* data;
    };
    Feed* get_feed(const char*);
    FeedInfo* get_feedinfo(Feed*);
    void release_feedinfo(FeedInfo**);
    void release_feed(Feed**);
#ifdef __cplusplus
}
#endif
