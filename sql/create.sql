drop table wishlist;
drop table own;
drop table history_price;
drop table game;
drop table user;

create table game(
    gamename varchar(200),
    oringinal_price float,
    current_price float,
    lowest_price float,
    link varchar(200),
    image_url varchar(200),
    desc text,

    primary key (gamename)
);

create table user(
    nickname varchar(200),
    email varchar(200),
    password varchar(200),

    primary key (nickname)
);

create table wishlist(
    nickname int,
    gamename int,
    email_alert boolean,

    primary key (nickname,gamename),
    foreign key (nickname) references user(nickname),
    foreign key (gamename) references game(gamename)
);

create table own(
    nickname int,
    gamename int,

    primary key (nickname,gamename),
    foreign key (nickname) references user(nickname),
    foreign key (gamename) references game(gamename)
);

create table history_price(
    gamename varchar(200),
    price float,
    date date,

    primary key (gamename,date),
    foreign key (gamename) references game(gamename)
);
