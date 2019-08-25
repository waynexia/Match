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
        oringinal_price->Float,
        current_price->Float,
        lowest_price->Float,
        link->Text,
        image_url->Text,
        desc->Text,
    }
}

table! {
    wishlist(nickname,gamename){
        nickname->Integer,
        gamename->Integer,
        email_alert->Bool,
    }
}
