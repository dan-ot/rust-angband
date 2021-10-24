use crate::types::Loc;
use crate::objects::Object;

enum game_event_type
{
	EVENT_MAP = 0,		/* Some part of the map has changed. */

	EVENT_STATS,  		/* One or more of the stats. */
	EVENT_HP,	   	/* HP or MaxHP. */
	EVENT_MANA,		/* Mana or MaxMana. */
	EVENT_AC,		/* Armour Class. */
	EVENT_EXPERIENCE,	/* Experience or MaxExperience. */
	EVENT_PLAYERLEVEL,	/* Player's level has changed */
	EVENT_PLAYERTITLE,	/* Player's title has changed */
	EVENT_GOLD,		/* Player's gold amount. */
	EVENT_MONSTERHEALTH,	/* Observed monster's health level. */
	EVENT_DUNGEONLEVEL,	/* Dungeon depth */
	EVENT_PLAYERSPEED,	/* Player's speed */
	EVENT_RACE_CLASS,	/* Race or Class */
	EVENT_STUDYSTATUS,	/* "Study" availability */
	EVENT_STATUS,		/* Status */
	EVENT_DETECTIONSTATUS,  /* Trap detection status */
	EVENT_FEELING,		/* Object level feeling */
	EVENT_LIGHT,		/* Light level */
	EVENT_STATE,		/* The two 'R's: Resting and Repeating */

	EVENT_PLAYERMOVED,
	EVENT_SEEFLOOR,         /* When the player would "see" floor objects */
	EVENT_EXPLOSION,
	EVENT_BOLT,
	EVENT_MISSILE,

	EVENT_INVENTORY,
	EVENT_EQUIPMENT,
	EVENT_ITEMLIST,
	EVENT_MONSTERLIST,
	EVENT_MONSTERTARGET,
	EVENT_OBJECTTARGET,
	EVENT_MESSAGE,
	EVENT_SOUND,
	EVENT_BELL,
	EVENT_USE_STORE,
	EVENT_STORECHANGED,	/* Triggered on a successful buy/retrieve or sell/drop */

	EVENT_INPUT_FLUSH,
	EVENT_MESSAGE_FLUSH,
	EVENT_CHECK_INTERRUPT,
	EVENT_REFRESH,
	EVENT_NEW_LEVEL_DISPLAY,
	EVENT_COMMAND_REPEAT,
	EVENT_ANIMATE,
	EVENT_CHEAT_DEATH,

	EVENT_INITSTATUS,	/* New status message for initialisation */
	EVENT_BIRTHPOINTS,	/* Change in the birth points */

	/* Changing of the game state/context. */
	EVENT_ENTER_INIT,
	EVENT_LEAVE_INIT,
	EVENT_ENTER_BIRTH,
	EVENT_LEAVE_BIRTH,
	EVENT_ENTER_GAME,
	EVENT_LEAVE_GAME,
	EVENT_ENTER_WORLD,
	EVENT_LEAVE_WORLD,
	EVENT_ENTER_STORE,
	EVENT_LEAVE_STORE,
	EVENT_ENTER_DEATH,
	EVENT_LEAVE_DEATH,

	/* Events for introspection into dungeon generation */
	EVENT_GEN_LEVEL_START, /* has string in event data for profile name */
	EVENT_GEN_LEVEL_END, /* has flag in event data indicating success */
	EVENT_GEN_ROOM_START, /* has string in event data for room type */
	EVENT_GEN_ROOM_CHOOSE_SIZE, /* has size in event data */
	EVENT_GEN_ROOM_CHOOSE_SUBTYPE, /* has string in event data with name */
	EVENT_GEN_ROOM_END, /* has flag in event data indicating success */
	EVENT_GEN_TUNNEL_FINISHED, /* has tunnel in event data with results */

	EVENT_END  /* Can be sent at the end of a series of events */
}

pub struct Message {
	msg: String,
	msg_type: i32
}

pub struct Birthstage<T> {
	reset: bool,
	hint: String,
	n_choices: i32,
	initial_choice: i32,
	choices: Vec<String>,
	helptexts: Vec<String>,
	xtra: T
}

pub struct Birthpoints {
	points: Vec<i32>,
	inc_points: Vec<i32>,
	remaining: i32
}

pub struct Explosion {
	proj_type: i32,
	num_grids: i32,
	distance_to_grid: Vec<i32>,
	drawing: bool,
	player_sees_grid: Vec<bool>,
	blast_grid: Vec<Loc>,
	centre: Loc
}

pub struct Bolt {
	proj_type: i32,
	drawing: bool,
	seen: bool,
	beam: bool,
	oy: i32,
	ox: i32,
	y: i32,
	x: i32,
}

pub struct Missile {
	obj: Object,
	seen: bool,
	y: i32,
	x: i32
}

pub struct Size {
	h: i32,
	w: i32
}

pub struct Tunnel {
	/// "nstep" is the total number of tunneling steps made
	nstep: i32,
	/// "npierce" is the total number of wall piercings for rooms
	npierce: i32,
	///"ndug" is the number of tiles excavated (excluding wall piercings).
	ndug: i32,
	/** "dstart" is the city block distance, in grids, (i.e.
		ABS(start.x - end.x) + ABS(start.y - end.y)) between the
		startng point and the goal for the tunnel. */
	dstart: i32,
	/** "dend" is the
		city block distance between the final point in the tunnel
		and the goal:  "dend" equal to zero indicates that the
		tunnel reached its goal. */
	dend: i32,
	/** "early" is true if the tunnel was terminated by the random early termination criteria. */
	early: bool
}

pub enum GameEventData<T> {
	POINT(Loc),

	STRING(String),

	FLAG(bool),

	MESSAGE(Message),

	BIRTHSTAGE(Birthstage<T>),

	BIRTHPOINTS(Birthpoints),

	EXPLOSION(Explosion),

	BOLT(Bolt),

	MISSILE(Missile),

	SIZE(Size),

	TUNNEL(Tunnel)
}
