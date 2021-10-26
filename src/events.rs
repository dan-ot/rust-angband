use std::collections::HashMap;
use crate::types::Loc;
use crate::objects::Object;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GameEvent
{
	EventMap = 0,		/* Some part of the map has changed. */

	EventStats,  		/* One or more of the stats. */
	EventHp,	   	/* HP or MaxHP. */
	EventMana,		/* Mana or MaxMana. */
	EventAc,		/* Armour Class. */
	EventExperience,	/* Experience or MaxExperience. */
	EventPlayerlevel,	/* Player's level has changed */
	EventPlayertitle,	/* Player's title has changed */
	EventGold,		/* Player's gold amount. */
	EventMonsterhealth,	/* Observed monster's health level. */
	EventDungeonlevel,	/* Dungeon depth */
	EventPlayerspeed,	/* Player's speed */
	EventRaceClass,	/* Race or Class */
	EventStudystatus,	/* "Study" availability */
	EventStatus,		/* Status */
	EventDetectionstatus,  /* Trap detection status */
	EventFeeling,		/* Object level feeling */
	EventLight,		/* Light level */
	EventState,		/* The two 'R's: Resting and Repeating */

	EventPlayermoved,
	EventSeefloor,         /* When the player would "see" floor objects */
	EventExplosion,
	EventBolt,
	EventMissile,

	EventInventory,
	EventEquipment,
	EventItemlist,
	EventMonsterlist,
	EventMonstertarget,
	EventObjecttarget,
	EventMessage,
	EventSound,
	EventBell,
	EventUseStore,
	EventStorechanged,	/* Triggered on a successful buy/retrieve or sell/drop */

	EventInputFlush,
	EventMessageFlush,
	EventCheckInterrupt,
	EventRefresh,
	EventNewLevelDisplay,
	EventCommandRepeat,
	EventAnimate,
	EventCheatDeath,

	EventInitstatus,	/* New status message for initialisation */
	EventBirthpoints,	/* Change in the birth points */

	/* Changing of the game state/context. */
	EventEnterInit,
	EventLeaveInit,
	EventEnterBirth,
	EventLeaveBirth,
	EventEnterGame,
	EventLeaveGame,
	EventEnterWorld,
	EventLeaveWorld,
	EventEnterStore,
	EventLeaveStore,
	EventEnterDeath,
	EventLeaveDeath,

	/* Events for introspection into dungeon generation */
	EventGenLevelStart, /* has string in event data for profile name */
	EventGenLevelEnd, /* has flag in event data indicating success */
	EventGenRoomStart, /* has string in event data for room type */
	EventGenRoomChooseSize, /* has size in event data */
	EventGenRoomChooseSubtype, /* has string in event data with name */
	EventGenRoomEnd, /* has flag in event data indicating success */
	EventGenTunnelFinished, /* has tunnel in event data with results */

	EventEnd  /* Can be sent at the end of a series of events */
}

#[derive(Debug, Clone)]
pub struct Message {
	msg: String,
	msg_type: i32
}

#[derive(Debug, Clone)]
pub struct Birthstage {
	reset: bool,
	hint: String,
	n_choices: i32,
	initial_choice: i32,
	choices: Vec<String>,
	helptexts: Vec<String>,
	xtra: bool
}

#[derive(Debug, Clone)]
pub struct Birthpoints {
	points: Vec<i32>,
	inc_points: Vec<i32>,
	remaining: i32
}

#[derive(Debug, Clone)]
pub struct Explosion {
	proj_type: i32,
	num_grids: i32,
	distance_to_grid: Vec<i32>,
	drawing: bool,
	player_sees_grid: Vec<bool>,
	blast_grid: Vec<Loc>,
	centre: Loc
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Missile {
	obj: Object,
	seen: bool,
	y: i32,
	x: i32
}

#[derive(Debug, Clone)]
pub struct Size {
	h: i32,
	w: i32
}

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub enum GameEventData {
	Point (Loc),

	String (String),

	Flag (bool),

	Message (Message),

	Birthstage (Birthstage),

	Birthpoints (Birthpoints),

	Explosion (Explosion),

	Bolt (Bolt),

	Missile (Missile),

	Size (Size),

	Tunnel (Tunnel)
}

pub struct EventContext {

}

pub struct MessageEntry {
	pub context: EventContext,
	pub operation: Box<dyn FnMut(&GameEvent, &GameEventData, &mut EventContext) -> ()>
}

pub struct MessageQueue {
	event_handlers: HashMap<GameEvent, Vec<MessageEntry>>
}

impl MessageQueue {
	pub fn new() -> MessageQueue {
		let mut map = HashMap::new();
		map.insert(GameEvent::EventMap, vec!());

		map.insert(GameEvent::EventStats, vec!());
		map.insert(GameEvent::EventHp, vec!());
		map.insert(GameEvent::EventMana, vec!());
		map.insert(GameEvent::EventAc, vec!());
		map.insert(GameEvent::EventExperience, vec!());
		map.insert(GameEvent::EventPlayerlevel, vec!());
		map.insert(GameEvent::EventPlayertitle, vec!());
		map.insert(GameEvent::EventGold, vec!());
		map.insert(GameEvent::EventMonsterhealth, vec!());
		map.insert(GameEvent::EventDungeonlevel, vec!());
		map.insert(GameEvent::EventPlayerspeed, vec!());
		map.insert(GameEvent::EventRaceClass, vec!());
		map.insert(GameEvent::EventStudystatus, vec!());
		map.insert(GameEvent::EventStatus, vec!());
		map.insert(GameEvent::EventDetectionstatus, vec!());
		map.insert(GameEvent::EventFeeling, vec!());
		map.insert(GameEvent::EventLight, vec!());
		map.insert(GameEvent::EventState, vec!());
		map.insert(GameEvent::EventPlayermoved, vec!());
		map.insert(GameEvent::EventSeefloor, vec!());
		map.insert(GameEvent::EventExplosion, vec!());
		map.insert(GameEvent::EventBolt, vec!());
		map.insert(GameEvent::EventMissile, vec!());
		map.insert(GameEvent::EventInventory, vec!());
		map.insert(GameEvent::EventEquipment, vec!());
		map.insert(GameEvent::EventItemlist, vec!());
		map.insert(GameEvent::EventMonsterlist, vec!());
		map.insert(GameEvent::EventMonstertarget, vec!());
		map.insert(GameEvent::EventObjecttarget, vec!());
		map.insert(GameEvent::EventMessage, vec!());
		map.insert(GameEvent::EventSound, vec!());
		map.insert(GameEvent::EventBell, vec!());
		map.insert(GameEvent::EventUseStore, vec!());
		map.insert(GameEvent::EventStorechanged, vec!());
		map.insert(GameEvent::EventInputFlush, vec!());
		map.insert(GameEvent::EventMessageFlush, vec!());
		map.insert(GameEvent::EventCheckInterrupt, vec!());
		map.insert(GameEvent::EventRefresh, vec!());
		map.insert(GameEvent::EventNewLevelDisplay, vec!());
		map.insert(GameEvent::EventCommandRepeat, vec!());
		map.insert(GameEvent::EventAnimate, vec!());
		map.insert(GameEvent::EventCheatDeath, vec!());
		map.insert(GameEvent::EventInitstatus, vec!());
		map.insert(GameEvent::EventBirthpoints, vec!());
		map.insert(GameEvent::EventEnterInit, vec!());
		map.insert(GameEvent::EventLeaveInit, vec!());
		map.insert(GameEvent::EventEnterBirth, vec!());
		map.insert(GameEvent::EventLeaveBirth, vec!());
		map.insert(GameEvent::EventEnterGame, vec!());
		map.insert(GameEvent::EventLeaveGame, vec!());
		map.insert(GameEvent::EventEnterWorld, vec!());
		map.insert(GameEvent::EventLeaveWorld, vec!());
		map.insert(GameEvent::EventEnterStore, vec!());
		map.insert(GameEvent::EventLeaveStore, vec!());
		map.insert(GameEvent::EventEnterDeath, vec!());
		map.insert(GameEvent::EventLeaveDeath, vec!());
		map.insert(GameEvent::EventGenLevelStart, vec!());
		map.insert(GameEvent::EventGenLevelEnd, vec!());
		map.insert(GameEvent::EventGenRoomStart, vec!());
		map.insert(GameEvent::EventGenRoomChooseSize, vec!());
		map.insert(GameEvent::EventGenRoomChooseSubtype, vec!());
		map.insert(GameEvent::EventGenRoomEnd, vec!());
		map.insert(GameEvent::EventGenTunnelFinished, vec!());
		map.insert(GameEvent::EventEnd, vec!());

		MessageQueue {
			event_handlers: map
		}
	}

	pub fn dispatch(&mut self, event_type: &GameEvent, event_data: GameEventData) -> () {
		let mut handlers = self.event_handlers.get(&event_type).expect("Couldn't find an event type, should have been initialized!");
		todo!();
		// for handler in handlers {
		// 	(handler.operation)(&event_type, &event_data, &handler.context);
		// }
	}
}