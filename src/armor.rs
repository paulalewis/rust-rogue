use crate::rogue::*;

/*
// The player wants to wear something, so let him/her put it on.
void
wear()
{
    register THING *obj;
    register char *sp;

    if ((obj = get_item("wear", ARMOR)) == NULL)
	return;
    if (cur_armor != NULL)
    {
	addmsg("you are already wearing some.  You'll have to take it off first");
	endmsg();
	after = FALSE;
	return;
    }
    if (obj->o_type != ARMOR)
    {
	msg("you can't wear that");
	return;
    }
    waste_time();
    obj->o_flags |= ISKNOW;
    sp = inv_name(obj, TRUE);
    cur_armor = obj;
    msg("you are now wearing %s", sp);
}

/*
 *	Get the armor off of the players back
 */
void
take_off()
{
    register THING *obj;

    if ((obj = cur_armor) == NULL)
    {
	after = FALSE;
	msg("you aren't wearing any armor");
	return;
    }
    if (!dropcheck(cur_armor))
	return;
    cur_armor = NULL;
    msg("you used to be wearing %c) %s", obj->o_packch, inv_name(obj, TRUE));
}

/*
 * waste_time:
 *	Do nothing but let other things happen
 */
void
waste_time()
{
    do_daemons(BEFORE);
    do_fuses(BEFORE);
    do_daemons(AFTER);
    do_fuses(AFTER);
}
*/