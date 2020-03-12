# Dhai'procol

From the old tongue, "Maps of War," or `procol` for short.

Procol is a mud client specialized for players of the Wheel of Time mud at wotmud.org.

## Goals

While first and foremost this project is a fun project for its author, me, there are some general goals for the project itself.

### Amazing Cartography

Procol will excel at keeping track of where you are in the mud and helping you to create maps of the places you've been.

### Collaborative Cartography

I want you to be able to easily share your maps with friends while also keeping some data hidden.

### Crazy fast

Expensive calculations and processing should be run in the background so the mud itself is fully responsive to your keystrokes and network input. Incoming text should not be delayed because of slow-running scripts whenever possible.

For those of you with a programming background, Procol uses background threads to do things like path finding without interrupting your ability to interact with the mud.

### Smart and safe scripting

Because I'm a glutton for punishment I want to implement "safe and simple" user scripting that will be accessible to beginner programmers without sacrificing the speed or safety of the client.

Advanced features should be opt-in, so easy things should be easy to do, and difficult things should be possible.

Scripts should be sand-boxed so it becomes safe(er) to share scripts with your friends.

### Cryptographic signatures

When sharing maps and/or scripts, it's nice to know where they came from. When you write a script and share it, you sign it so they know it came from you. If they modify it and re-share it, others can see their changes as well as the fact that it originally came from you.

Knowing the history of maps and scripts should help you manage your circles of trust better as you explore into unfamiliar terrorities both in the mud and in the land of scripting.
