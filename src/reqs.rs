use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct DictContentStruct {
    word: String,
    description: String,
    id: u32,
}
#[derive(Serialize, Deserialize)]
struct SearchString {
    word: String,
    description: String,
}

enum DictContent {
    DictContentStruct,
    SearchString,
}

#[derive(Serialize, Deserialize)]
struct MyReq {
    reqtype: String,      // Either 'get' or 'push'
    content: DictContent, // search string or content to return
}
