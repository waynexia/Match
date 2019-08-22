table! {
    user (nickname){
        nickname -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

table! {
    game(gamename){
        gamename->Varchar,
        price->Float,
        link->Text,
        image_url->Text,
        desc->Text,
    }
}
