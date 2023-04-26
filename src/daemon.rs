use crate::rogue::*;

const EMPTY: usize = 0;
const DAEMON: isize = -1;

/*
#define _X_ { EMPTY }

struct delayed_action d_list[MAXDAEMONS] = {
    _X_, _X_, _X_, _X_, _X_, _X_, _X_, _X_, _X_, _X_,
    _X_, _X_, _X_, _X_, _X_, _X_, _X_, _X_, _X_, _X_, 
};

// Find an empty slot in the daemon/fuse list
struct delayed_action * d_slot() {
    register struct delayed_action *dev;

    for (dev = d_list; dev <= &d_list[MAXDAEMONS-1]; dev++)
	if (dev->d_type == EMPTY)
	    return dev;
    return NULL;
}

// Find a particular slot in the table
struct delayed_action * find_slot(void (*func)()) {
    register struct delayed_action *dev;

    for (dev = d_list; dev <= &d_list[MAXDAEMONS-1]; dev++)
	if (dev->d_type != EMPTY && func == dev->d_func)
	    return dev;
    return NULL;
}

// Start a daemon, takes a function.
void start_daemon(void (*func)(), int arg, int type) {
    register struct delayed_action *dev;

    dev = d_slot();
    dev->d_type = type;
    dev->d_func = func;
    dev->d_arg = arg;
    dev->d_time = DAEMON;
}

// Remove a daemon from the list
void kill_daemon(void (*func)()) {
    register struct delayed_action *dev;

    if ((dev = find_slot(func)) == NULL)
	return;
    /*
     * Take it out of the list
     */
    dev->d_type = EMPTY;
}

// Run all the daemons that are active with the current flag,
// passing the argument to the function.
void do_daemons(int flag) {
    register struct delayed_action *dev;

    /*
     * Loop through the devil list
     */
    for (dev = d_list; dev <= &d_list[MAXDAEMONS-1]; dev++)
	/*
	 * Executing each one, giving it the proper arguments
	 */
	if (dev->d_type == flag && dev->d_time == DAEMON)
	    (*dev->d_func)(dev->d_arg);
}
*/
pub fn do_daemons(flag: usize) {
}

/*
// Start a fuse to go off in a certain number of turns
void fuse(void (*func)(), int arg, int time, int type) {
    register struct delayed_action *wire;

    wire = d_slot();
    wire->d_type = type;
    wire->d_func = func;
    wire->d_arg = arg;
    wire->d_time = time;
}

// Increase the time until a fuse goes off
void lengthen(void (*func)(), int xtime) {
    register struct delayed_action *wire;

    if ((wire = find_slot(func)) == NULL)
	return;
    wire->d_time += xtime;
}

// Put out a fuse
void extinguish(void (*func)()) {
    register struct delayed_action *wire;

    if ((wire = find_slot(func)) == NULL)
	return;
    wire->d_type = EMPTY;
}

// Decrement counters and start needed fuses
void do_fuses(int flag) {
    register struct delayed_action *wire;

    /*
     * Step though the list
     */
    for (wire = d_list; wire <= &d_list[MAXDAEMONS-1]; wire++)
	/*
	 * Decrementing counters and starting things we want.  We also need
	 * to remove the fuse from the list once it has gone off.
	 */
	if (flag == wire->d_type && wire->d_time > 0 && --wire->d_time == 0)
	{
	    wire->d_type = EMPTY;
	    (*wire->d_func)(wire->d_arg);
	}
}
*/
pub fn do_fuses(flag: usize) {
}