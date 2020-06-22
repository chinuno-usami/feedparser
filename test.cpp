#include <time.h>
#include <iostream>
#include "feedparser.h"
using namespace std;

#include <string>
std::string feedstring2string(FeedString& str){
    return std::string(str.data, str.size);
}

int main(){
    FeedInfo* info = feedparser_parse_url("https://rsshub.app/telegram/channel/awesomeDIYgod");
    if(!info){
        cout << "get feed failed\n";
        return 1;
    }

    std::string s(info->title.data, info->title.size);
    cout << "title:" << s<<endl;
    cout << "update:" << info->updated <<endl;
    cout << "size:" << info->size <<endl;
    for(size_t i = 0; i < info->size; ++i){
        FeedEntry& entry = info->entries[i];
        if(entry.title.size){
            cout<<"  title:" << feedstring2string(entry.title) << endl;
        }
        cout << "  updated:" << entry.updated << endl;
        cout << "  published:" << entry.published << endl;
        if(entry.id.size){
            cout<<"  id:" << feedstring2string(entry.id) << endl;
        }
        if(entry.link.size){
            cout<<"  link:" << feedstring2string(entry.link) << endl;
        }
        if(entry.summary.size){
            cout<<"  summary:" << feedstring2string(entry.summary) << endl;
        }
        cout << endl;
    }

    feedparser_release_feedinfo(&info);

}
