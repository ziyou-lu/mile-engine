-- --------------------------------------------------------------------
-- 玩家数据表
-- --------------------------------------------------------------------
drop table if exists player_data;
create table player_data (
    uuid varchar(128) NOT NULL,
    data text NOT NULL,
    primary key (uuid)
);

-- ----------------------------------------------------------------
-- 场景数据表
-- ----------------------------------------------------------------
drop table if exists scene_data;
create table scene_data (
    scene_id integer not null,
    data text not null,
    primary key (scene_id)
);