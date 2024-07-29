use crate::{core::{constants::{ISHELD, MAXTRAPS}, rogue_state::RogueState, utils::rnd}, core::things::create_object};

use std::cmp::max;

use super::{constants::{AMULET_LEVEL, FLOOR, F_REAL, MAXOBJ, NTRAPS}, object::Object, object_type::ObjectType, passages::do_passages, rooms::{do_rooms, find_random_empty_floor_on_map}};

// one chance in CHANCE_OF_TREASURE_ROOM for a treasure room
const CHANCE_OF_TREASURE_ROOM: usize = 20;
const MAX_TREASURES: usize = 10;
const MIN_TREASURES: usize = 2;
// max number of tries to put down a monster
const MAX_TRIES: usize =  10;

/*
void new_level() {
    THING *tp;
    PLACE *pp;
    char *sp;
    int i;

    /*
     * Clean things off from last level
     */
    for (pp = places; pp < &places[SCREEN_WIDTH*SCREEN_HEIGHT]; pp++)
    {
	pp->p_ch = ' ';
	pp->p_flags = F_REAL;
	pp->p_monst = NULL;
    }
    clear();
    /*
     * Free up the monsters on the last level
     */
    for (tp = mlist; tp != NULL; tp = next(tp))
	free_list(tp->t_pack);
    free_list(mlist);
    /*
     * Throw away stuff left on the previous level (if anything)
     */
    free_list(lvl_obj);
    do_rooms();				/* Draw rooms */
    do_passages();			/* Draw passages */
    no_food++;
    put_things();			/* Place objects (if any) */
    /*
     * Place the traps
     */
    if (rnd(10) < level)
    {
	ntraps = rnd(level / 4) + 1;
	if (ntraps > MAXTRAPS)
	    ntraps = MAXTRAPS;
	i = ntraps;
	while (i--)
	{
	    /*
	     * not only wouldn't it be NICE to have traps in mazes
	     * (not that we care about being nice), since the trap
	     * number is stored where the passage number is, we
	     * can't actually do it.
	     */
	    do
	    {
		find_floor((struct room *) NULL, &stairs, FALSE, FALSE);
	    } while (chat(stairs.y, stairs.x) != FLOOR);
	    sp = &flat(stairs.y, stairs.x);
	    *sp &= ~F_REAL;
	    *sp |= rnd(NTRAPS);
	}
    }
    /*
     * Place the staircase down.
     */
    find_floor((struct room *) NULL, &stairs, FALSE, FALSE);
    chat(stairs.y, stairs.x) = STAIRS;
    seenstairs = FALSE;

    for (tp = mlist; tp != NULL; tp = next(tp))
	tp->t_room = roomin(&tp->t_pos);

    find_floor((struct room *) NULL, &hero, FALSE, TRUE);
    enter_room(&hero);
    mvaddch(hero.y, hero.x, PLAYER);
    if (on(player, SEEMONST))
	turn_see(FALSE);
    if (on(player, ISHALU))
	visuals();
}*/
pub fn new_level(state: &mut RogueState) {
    do_rooms(state);
    do_passages(state);
    state.no_food += 1;
    put_things(state);
    place_traps(state);
    place_stairs_down(state);

    // for (tp = mlist; tp != NULL; tp = next(tp))
	// tp->t_room = roomin(&tp->t_pos);

    // find_floor((struct room *) NULL, &hero, FALSE, TRUE);
    // enter_room(&hero);
    // mvaddch(hero.y, hero.x, PLAYER);
    // if (on(player, SEEMONST))
	// turn_see(FALSE);
    // if (on(player, ISHALU))
	// visuals();
    
    //player.t_flags &= ~ISHELD;	/* unhold when you go down just in case */
    state.player.player_stats.flags &= !ISHELD;
    if state.dungeon.level > state.max_level {
        state.max_level = state.dungeon.level;
    }
}

fn place_traps(state: &mut RogueState) {
    if rnd(10) < state.dungeon.level {
        let ntraps = max(state.dungeon.level / 4 + 1, MAXTRAPS);
        for _ in 0..ntraps {
            let floor = loop {
                let coord = find_random_empty_floor_on_map(state, 0, false).unwrap();
                if state.dungeon.get_place(coord).ch == FLOOR {
                    break coord;
                }
            };
            let place = state.dungeon.get_place_mut(floor);
            place.flags &= !F_REAL;
            place.flags |= rnd(NTRAPS);
        }
    }
}

fn place_stairs_down(state: &mut RogueState) {
    state.dungeon.stairs = find_random_empty_floor_on_map(state, 0, false).unwrap();
    state.seenstairs = false;
}

/*
void
put_things()
{
    int i;
    THING *obj;

    /*
     * Once you have found the amulet, the only way to get new stuff is
     * go down into the dungeon.
     */
    if (amulet && level < max_level)
	return;
    /*
     * check for treasure rooms, and if so, put it in.
     */
    if (rnd(CHANCE_OF_TREASURE_ROOM) == 0)
	treas_room();
    /*
     * Do MAXOBJ attempts to put things on a level
     */
    for (i = 0; i < MAXOBJ; i++)
	if (rnd(100) < 36)
	{
	    /*
	     * Pick a new object and link it in the list
	     */
	    obj = new_thing();
	    attach(lvl_obj, obj);
	    /*
	     * Put it somewhere
	     */
	    find_floor((struct room *) NULL, &obj->o_pos, FALSE, FALSE);
	    chat(obj->o_pos.y, obj->o_pos.x) = (char) obj->o_type;
	}
    /*
     * If he is really deep in the dungeon and he hasn't found the
     * amulet yet, put it somewhere on the ground
     */
    if (level >= AMULETLEVEL && !amulet)
    {
	obj = new_item();
	attach(lvl_obj, obj);
	obj->o_hplus = 0;
	obj->o_dplus = 0;
	strncpy(obj->o_damage,"0x0",sizeof(obj->o_damage));
        strncpy(obj->o_hurldmg,"0x0",sizeof(obj->o_hurldmg));
	obj->o_arm = 11;
	obj->o_type = AMULET;
	/*
	 * Put it somewhere
	 */
	find_floor((struct room *) NULL, &obj->o_pos, FALSE, FALSE);
	chat(obj->o_pos.y, obj->o_pos.x) = AMULET;
    }
}
*/
/// Put potions and scrolls on this level
pub fn put_things(state: &mut RogueState) {
    // Once you have found the amulet, the only way to get new stuff is
    // go down into the dungeon.
    if state.amulet && state.dungeon.level < state.max_level { return; }

    // check for treasure rooms, and if so, put it in.
    if rnd(CHANCE_OF_TREASURE_ROOM) == 0 { add_treasure_room(state); }

    // Do MAXOBJ attempts to put things on a level
    for i in 0..MAXOBJ {
        if rnd(100) < 36 {
            // Pick a new object and link it in the list
            let obj = create_object(state);
            state.dungeon.objects.push(obj);
            // Put it somewhere
            // find_floor(None, &obj.pos, false, false);
            // state.dungeon.place_at(obj.pos).ch = obj.object_type.map_character();, don't do, this should be done when map rendered
        }
    }

    // If he is really deep in the dungeon and he hasn't found the
    // amulet yet, put it somewhere on the ground
    if state.dungeon.level >= AMULET_LEVEL && !state.amulet {
        let obj = Object::new(ObjectType::Amulet);
        state.dungeon.objects.push(obj);
        // Put it somewhere
        // find_floor((struct room *) NULL, &obj->o_pos, false, false);
        // state.dungeon.place_at(obj.pos).ch = obj.object_type.map_character();, don't do, this should be done when map rendered
    }
}

/*
void
treas_room()
{
    int nm;
    THING *tp;
    struct room *rp;
    int spots, num_monst;
    static coord mp;

    rp = &rooms[rnd_room()];
    spots = (rp->r_max.y - 2) * (rp->r_max.x - 2) - MIN_TREASURES;
    if (spots > (MAX_TREASURES - MIN_TREASURES))
	spots = (MAX_TREASURES - MIN_TREASURES);
    num_monst = nm = rnd(spots) + MIN_TREASURES;
    while (nm--)
    {
	find_floor(rp, &mp, 2 * MAX_TRIES, FALSE);
	tp = new_thing();
	tp->o_pos = mp;
	attach(lvl_obj, tp);
	chat(mp.y, mp.x) = (char) tp->o_type;
    }

    /*
     * fill up room with monsters from the next level down
     */

    if ((nm = rnd(spots) + MIN_TREASURES) < num_monst + 2)
	nm = num_monst + 2;
    spots = (rp->r_max.y - 2) * (rp->r_max.x - 2);
    if (nm > spots)
	nm = spots;
    level++;
    while (nm--)
    {
	spots = 0;
	if (find_floor(rp, &mp, MAX_TRIES, TRUE))
	{
	    tp = new_item();
	    new_monster(tp, randmonster(FALSE), &mp);
	    tp->t_flags |= ISMEAN;	/* no sloughers in THIS room */
	    give_pack(tp);
	}
    }
    level--;
}
*/
fn add_treasure_room(state: &mut RogueState) {
    todo!();
}