create table if not exists dogs
(
    desertion_no   varchar(15) primary key,
    filename       varchar(100) not null,
    image_path     varchar(100),
    happen_dt      date         not null,
    kind_cd        varchar(150) not null,
    color_cd       varchar(90)  not null,
    age            varchar(30)  not null,
    weight         varchar(30)  not null,
    sex_cd         varchar(15)  not null,
    neuter_yn      varchar(15)  not null,
    care_nm        varchar(150) not null,
    care_tel       varchar(30)  not null,
    care_addr      varchar(600) not null,
    charge_nm      varchar(60),
    officetel      varchar(30)  not null,
    notice_comment varchar(600)
);