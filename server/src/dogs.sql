create table if not exists dogs
(
    id            varchar(15) primary key,
    happen_dt      date         not null,
    kind_cd        varchar(150) not null,
    color_cd       varchar(90)  not null,
    age           int          not null,
    weight        int          not null,
    sex_cd         varchar(15)  not null,
    neuter_yn      varchar(15)  not null,
    care_nm        varchar(150) not null,
    care_tel       varchar(30)  not null,
    care_addr      varchar(600) not null,
    charge_nm      varchar(60)  not null,
    officetel     varchar(30)  not null,
    notice_comment varchar(600) not null
);