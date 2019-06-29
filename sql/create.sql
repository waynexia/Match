drop table want;
drop table own;
drop table history_price;
drop table game;
drop table user;

create table game(
    gamename varchar(200),
    price float,
    disc text,
    link varchar(200),
    image blob,

    primary key (gamename)
);

create table user(
    nickname varchar(200),
    email varchar(200),
    password varchar(200),

    primary key (nickname)
);

create table want(
    nickname varchar(200),
    gamename varchar(200),

    primary key (nickname,gamename),
    foreign key (nickname) references user(nickname),
    foreign key (gamename) references game(gamename)
);

create table own(
    nickname varchar(200),
    gamename varchar(200),

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
