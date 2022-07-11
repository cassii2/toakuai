CREATE TABLE users (
	id            UUID NOT NULL PRIMARY KEY,
	username      VARCHAR(64) NOT NULL,
	UNIQUE (username)
);

CREATE TABLE words (
	id            UUID NOT NULL PRIMARY KEY,
	author        UUID REFERENCES users ON DELETE CASCADE,
	word          TEXT NOT NULL,
	definition    TEXT NOT NULL,
	forked_from   UUID REFERENCES words ON DELETE SET NULL,
	lang          VARCHAR(8) NOT NULL,
	gloss         TEXT[] NOT NULL,
	frame         VARCHAR(3)[] NOT NULL,
	created       TIMESTAMP NOT NULL,
	edited        TIMESTAMP
);

CREATE TABLE comments (
	id             UUID NOT NULL PRIMARY KEY,
	author         UUID NOT NULL REFERENCES users ON DELETE CASCADE,
	parent_word    UUID NOT NULL REFERENCES words ON DELETE CASCADE,
	parent_comment UUID REFERENCES comments ON DELETE CASCADE,
	content        TEXT NOT NULL
);
CREATE INDEX comments_parent_word_index ON comments (parent_word);
CREATE INDEX comments_parent_comment_index ON comments (parent_comment);

CREATE TABLE votes (
	author        UUID NOT NULL REFERENCES users ON DELETE CASCADE,
	entry_word    UUID REFERENCES words ON DELETE CASCADE,
	entry_comment UUID REFERENCES comments ON DELETE CASCADE,
	is_upvote     BOOLEAN NOT NULL,
	UNIQUE (author, entry_word, entry_comment)
);
CREATE INDEX votes_entry_word_index ON votes (entry_word);
CREATE INDEX votes_entry_comment_index ON votes (entry_comment);
