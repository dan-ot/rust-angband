use crate::messages::MessageType;
use std::collections::HashMap;
use std::iter::FromIterator;
use crate::types::Loc;
use crate::objects::Object;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GameEvent
{
	/// Some part of the map has changed.
	EventMap = 0,		

	/// One or more of the stats.
	EventStats,  		
	/// HP or MaxHP.
	EventHp,	   	
	/// Mana or MaxMana.
	EventMana,		
	/// Armour Class.
	EventAc,		
	/// Experience or MaxExperience.
	EventExperience,	
	/// Player's level has changed
	EventPlayerlevel,	
	/// Player's title has changed
	EventPlayertitle,	
	/// Player's gold amount.
	EventGold,		
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
	pub msg: Option<String>,
	pub msg_type: MessageType
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
		let mut map = HashMap::from_iter([
			(GameEvent::EventMap, vec!()),	
			(GameEvent::EventStats, vec!()),
			(GameEvent::EventHp, vec!()),
			(GameEvent::EventMana, vec!()),
			(GameEvent::EventAc, vec!()),
			(GameEvent::EventExperience, vec!()),
			(GameEvent::EventPlayerlevel, vec!()),
			(GameEvent::EventPlayertitle, vec!()),
			(GameEvent::EventGold, vec!()),
			(GameEvent::EventMonsterhealth, vec!()),
			(GameEvent::EventDungeonlevel, vec!()),
			(GameEvent::EventPlayerspeed, vec!()),
			(GameEvent::EventRaceClass, vec!()),
			(GameEvent::EventStudystatus, vec!()),
			(GameEvent::EventStatus, vec!()),
			(GameEvent::EventDetectionstatus, vec!()),
			(GameEvent::EventFeeling, vec!()),
			(GameEvent::EventLight, vec!()),
			(GameEvent::EventState, vec!()),
			(GameEvent::EventPlayermoved, vec!()),
			(GameEvent::EventSeefloor, vec!()),
			(GameEvent::EventExplosion, vec!()),
			(GameEvent::EventBolt, vec!()),
			(GameEvent::EventMissile, vec!()),
			(GameEvent::EventInventory, vec!()),
			(GameEvent::EventEquipment, vec!()),
			(GameEvent::EventItemlist, vec!()),
			(GameEvent::EventMonsterlist, vec!()),
			(GameEvent::EventMonstertarget, vec!()),
			(GameEvent::EventObjecttarget, vec!()),
			(GameEvent::EventMessage, vec!()),
			(GameEvent::EventSound, vec!()),
			(GameEvent::EventBell, vec!()),
			(GameEvent::EventUseStore, vec!()),
			(GameEvent::EventStorechanged, vec!()),
			(GameEvent::EventInputFlush, vec!()),
			(GameEvent::EventMessageFlush, vec!()),
			(GameEvent::EventCheckInterrupt, vec!()),
			(GameEvent::EventRefresh, vec!()),
			(GameEvent::EventNewLevelDisplay, vec!()),
			(GameEvent::EventCommandRepeat, vec!()),
			(GameEvent::EventAnimate, vec!()),
			(GameEvent::EventCheatDeath, vec!()),
			(GameEvent::EventInitstatus, vec!()),
			(GameEvent::EventBirthpoints, vec!()),
			(GameEvent::EventEnterInit, vec!()),
			(GameEvent::EventLeaveInit, vec!()),
			(GameEvent::EventEnterBirth, vec!()),
			(GameEvent::EventLeaveBirth, vec!()),
			(GameEvent::EventEnterGame, vec!()),
			(GameEvent::EventLeaveGame, vec!()),
			(GameEvent::EventEnterWorld, vec!()),
			(GameEvent::EventLeaveWorld, vec!()),
			(GameEvent::EventEnterStore, vec!()),
			(GameEvent::EventLeaveStore, vec!()),
			(GameEvent::EventEnterDeath, vec!()),
			(GameEvent::EventLeaveDeath, vec!()),
			(GameEvent::EventGenLevelStart, vec!()),
			(GameEvent::EventGenLevelEnd, vec!()),
			(GameEvent::EventGenRoomStart, vec!()),
			(GameEvent::EventGenRoomChooseSize, vec!()),
			(GameEvent::EventGenRoomChooseSubtype, vec!()),
			(GameEvent::EventGenRoomEnd, vec!()),
			(GameEvent::EventGenTunnelFinished, vec!()),
			(GameEvent::EventEnd, vec!()),
		]);

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