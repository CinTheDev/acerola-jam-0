# Acerola jam game

This is a game dedicated to the Acerola Jam 0.

The theme is ABERRATION.

## Final idea development

I've chosen to go with [Modifying laws of physics - Modified physics law - Changed formula / equation - Gravity does not decrease with distance]

### Background and "Lore"

The main concept will be that the general gravity equation will be changed from

F = G \* (m1 \* m2) / r^2

to

F = G \* m1 \* m2

#### Effects of the new equation

That means, distant celestial objects will have the same gravitational effect as earth does. (Gravity of earh is also affected directly.) In order to make the game more interesting by not immeadeatly dying from excessive gravitational effects, and to appeal to special relativity, this change will happen in the intro of the game, and that "gravitational bubble" expands at light speed. The moon will affect gravity in roughly 1.5 seconds, so it's effect belongs to the intro.

The time it takes for light to reach other planets or the sun is conveniently close to my intended gameplay time of a few minutes. Let's say, if the gravitational bubble reaches the sun (8 minutes), the game will be over. I could also shorten that time by declaring Mars or Venus the final planets. (The times here would be at least 4.2 minutes or 2.3 minutes). For simplicity, let's consider Venus the deadly one. And more conveniently, because earth doesn't have to be as close as possible to Venus, I can choose any time between 2.3 minutes and 8 minutes to remain realistic.

There's one problem though: Calculating the surface gravity using the new formula yields something like 4 \* 10^14 Newtons of force per kilogram, which is a bit extreme. This is effectively being inside an earth-massed black hole. Everything would immeadeatly be vaporized from the ginormous forces.

So, let's redefine that per celestial object, the effective gravity is measured at the surface, and not in the middle. This makes the earths gravity be unaffected, but slowly, as more objects affect the gravity, it slowly gets worse. Especially the moon will be felt at the beginning, with an 1.62 m / s^2 of additional acceleration.

The game plays at sunset, and Venus is roughly in line with the sun.

#### Lose and win condition

If the gravitational bubble reaches Venus, the player is launched to the side against a wall (towards the direction of the Sun / Venus). Some lab equipment will roll / fall onto the player, which is certain death. **This marks the lose condition**

**In order to win**, the player must recreate the same "event" that started all this in the first place. This will cause a second "gravitational bubble" to form around earth, with the old gravitation back. Lore-wise this second bubble will also expand, effectively creating an ever expanding 3d ring around earth, inside which this "false gravity" holds instead of regular gravity. This could cause mayhem in space, but it won't affect gameplay. At most it would affect the achieved ending, if there ever will be endings implemented.

#### Experiment

Now, what experiment caused this in the first place? It should be something quantum-related, since there's a similar real-life theory with elementary particles.

Quantum related stuff:

- Particle accelerators
- Quantum computers
- Something with quantum tunelling

Let's say, the player invented a device to "boost" quantum tunelling / to make it more probable. (Perhaps stuff like particle accelerators are involved, which could be relevant for the tasks later on). A quantum tunel causes an elementary particle (maybe Higgs-Boson because it is related to mass) to collapse into a more stable state, causing all the other stuff to unfold.

### Gameplay

We'll need some gameplay which goes well together with the time pressure.

The main idea is that the player needs to accomplish a series of tasks to advance further, and the goal is to recreate the same "reaction" which caused all this chaos. If the player fails to do all tasks within the defined time limit, the earth will be destroyed and the player will lose.

These tasks are a series of not very difficult puzzles and skill challenges, which are easy to fail if under pressure. There are two approaches for the difficulty:

- More difficult tasks, but without randomization
- Fairly easy tasks, but with randomization

The first creates a rewarding muscle-memory-based challenge similar to Celeste or Geometry Dash. There will definetly be a few repetitions before the placer can succeed. The latter creates some more diverse gameplay, which a player could definetly beat in the first try, or first few tries.

I'll go with the latter approach, as with a higher time limit (5 minutes) the player isn't likely to replay the game in this game jam setting. An average player should be able to beat the game first try if they invest some effort.

#### Tasks

Here is an idea collection of possible tasks to be done by the player, sorted by puzzles and challenges. There should be at least 5 total tasks, if the tasks take about 30 seconds on average.

##### Puzzles

###### Particle accelerator startup

The player will need to start and prime a particle accelerator. The terminal will display a "pipe rotation" challenge, where there is a grid of blocks, each block contains a "pipe", which is either straight or bent 90 degrees. The pipes need to connect a start point to an end point. This challenge will be hand created instead of procedual

- Player interacts with lever, activating the puzzle (perhaps it could light up or something)
- Player solves the puzzle, hole opens to put a container in.
- The player needs to bring a container and put it inside
- Once in, a button lights up
- Finally pressing the button, the particle accelerator starts up

###### Machine to create alloy

There's a machine that mixes lead and exotic matter into an alloy. There's no exotic matter in the lab, and it needs to be created by the machine. Exotic matter is atoms with extremely huge nuclei, so the machine needs to combine heavy elements somehow (lets say it creates billions of tons of pressure somehow). We use iron as the base, which has to be harvested from unused lab equipment.

- Find lead and put inside machine
- Find all iron equipment and throw into machine
- Start the process by activating a lever
- A minigame about rearranging atom nuclei appears on the screen
- After completing the minigame the "exotic" alloy is dispensed

##### Challenges

###### Quantum computer startup

The player needs to boot a quantum computer and execute a program on it. Instructions and password are on a sticky note on the display.

- Boot the computer
- Login with a cryptic password. The password is written on a sticky note on the display.
- The password is censored (*****), and if they fail to enter it correctly, the input will be deleted
- Once started, the player needs to locate "quantum-tunnel.exe" on their desktop and double click.

###### Prepping the Quantum Tunnel Device

This is the final step, preparing the Q.T.de before activating it. It needs to be cleaned up from dark matter from the previous attempt, and requires fresh alloy of lead and exotic matter. After that the quantum computer can start the experiment.

- Clean out dark matter
- Shove the alloy bar inside
- Start the quantum computer

## Idea development

The core game will be developed here. There will always be a list of ideas below each point, and then a subheader for exploring ideas further.

- Modifying the laws of physics
- Something parasitic
- Change in life
- Fitting into society
- Cooking game with suspicious ingredients

### Modifying the laws of physics

- A specific physics law is changed in some way (e.g. an equation), and the result must be analyzed so that interesting and hardly predictable gameplay mechanics can emerge from that simple change.
- A story on how the world has been destroyed by some kind of astronomical event, altering the world as we know it in some horrible way
- You are a person with a magic blessing / curse and have to continue life somehow (inspired from the tale of Midas)

#### Modified physics law

- Specific formula in dynamics (or it's bases like mechanics or kinematics) modified
- Laws of thermodynamics don't hold anymore, allowing impossible constructions to exploit infinite energy, or similar
- Chemistry becomes very constrained; elements become very unreactive
- Every object is locked in rotation, rotation is impossible without dissasembling and rearranging the object itself

##### Changed formula / equation

- Changing the way force behaves (F = m * a)
- Making gravity not decrease with distance

###### Force

*a = F / m* This is acceleration when a force is applied. What happens, when we choose to make acceleration exponential instead of linear? *a = e^F / m*

After some experimentation on paper I've decided to not do this. It's kind too complicated for this time window.

###### Gravity

*F = G \* (m1 \* m2) / r^2* into *F = G \* m1 \* m2* Your science experiment failed, and there's now a gravitaional wave spreading where the new equation holds instead of the old one. This wave travels at light speed, but it needs a few minutes before it reaches other planets and can cause mayhem. The goal is to stop this ASAP.

I'll go with this.

##### Broken thermodynamics

- First law no longer holds; energy can now be destroyed or created from nothing
- Second law no longer holds; energy can collect (e.g. cold flows into hot) instead of spreading out

#### Magic blessing / curse

- Everything you touch becomes gold (kinda familiar idea)
- A specific law of physics doesn't apply to you anymore (e.g. gravity, collision, ...)

### Something parasitic

- You are a parasite inside a host
- You are the host of parasites and have to get rid of them somehow

### Change in life

- You are just living your life, and an unexpected event redefines your world view, self image, and life goals.

### Fitting into society

- You are a member of society, and for some reason everybody has turned against you. Now you need to hide somehow, and survive.

### Cooking game with suspicious ingredients

- You are a master cook inside of a third world country, which has declared war, and now only have very limited ingredients with questionable quality. You must ensure that your customers don't die or experience other undesired side effects.
