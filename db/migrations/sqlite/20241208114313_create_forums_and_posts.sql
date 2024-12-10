CREATE TABLE
    forums (
        id INTEGER PRIMARY KEY NOT NULL,
        rkey TEXT NOT NULL,
        cid TEXT NOT NULL,
        name TEXT (300) NOT NULL,
        description TEXT (300)
    );

CREATE TABLE
    posts (
        id INTEGER PRIMARY KEY NOT NULL,
        rkey TEXT NOT NULL,
        cid TEXT NOT NULL
    );
