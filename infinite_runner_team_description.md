## Infinite Runner - Gallagher's Odyssey

## Canonical game repo URL: https://github.com/ckessler70/CS1666-InfiniteRunner

## Team Members
Advanced Topic Subteam 1: Procedural Generation

	* Caleb Kessler
		* Pitt ID: cdk46
		* GitHub Username: ckessler70

	* Dane Halle
		* Pitt ID: dmh148
		* GitHub Username: DaneHalle

	* Andrew Wiesen
		* Pitt ID: amw267
		* GitHub Username: drewwiesen155

	* Benjamin Ungar
		* Pitt ID: bhu4
		* GitHub Username: Bungarooo

Advanced Topic Subteam 2: Physics Engine

	* Dominic Karras
		* Pitt ID: dak198
		* GitHub Username: dak198

	* Mateen Kasim
		* Pitt ID: mak420
		* GitHub Username: mateenkasim

	* Elliot Snitzer
		* Pitt ID: efs24
		* GitHub Username: elliotsnitzer

	* Michael Daley
		* Pitt ID: mdd63
		* GitHub Username: mdaley6

## Game Description
"Gallagher's Odyssey" will be a Pitt-themed infinite sidescroller. Players will attempt to travel as far as possible across the randomly generated landscape using physics-based gameplay mechanics. There will be a single level which scales difficulty according to the distance traveled. The primary gameplay will be jumping and flipping while the player slides across the ground and flies through the air, such that the player may utilize the landscape to preserve or increase momentum. Additionally, a number of obstacles will be generated as part of the landscape which the player must avoid.

## Advanced Topic Descriptions

### Procedural Generation
The first advanced topic concerns 2D terrain generation, with hills and obstacles of various size as the primary features. Each time a new section of terrain is generated there will be a chance to produce a large obstacle instead of a hill. Small obstacles will be added to hills after they are created. All obstacles will be drawn from a pool of predefined options. Hills will be created with Bezier curves using Perlin noise to generate control points. Distance traveled will be used as input for hill generation and obstacle density in order to increase complexity and difficulty as the length of a run increases. 

### Physics Engine
The second advanced topic concerns the physics which will apply to both player and environment. The physics engine will implement a basic, rigid-body simulation of mass, acceleration, and collision for the player as well as other objects in the level. These entities will be able to freely move and rotate in response to collisions, gravity, and friction. Different objects will have different shapes, mass, and rotational inertia and will thus behave differently from each other. In implementing the buoyancy stretch goal, objects will also be assigned densities, which will govern how well they float atop water terrain.

## Midterm Goals
* A simple "base game" without the advanced topics implemented. Movement, game-ending collision, static obstacles, and collectible coins will be demonstrated within a testing environment.
* Demonstrable backend progress towards implementation of procedural terrain generation.
* Demonstrable backend progress towards implementation of a physics engine.

## Final Goals
* 20%: Basic game implementation such as start and end states, player movement, and accumulation of score.
* 25%: Procedural generation of terrain, obstacles, and coins such that gameplay may last as long as player skill allows.
* 25%: Physics engine with gravity, momentum, friction, and rigid body collisions such that gameplay may last as long as player skill allows.
* 5%: Menus, UI, and controls which allow players to easily interact with the game.

## Stretch Goals
* Power-Ups: Randomly generated, temporary modifications to the player or environment activated via a collectible.
	* **Score Booster** that doubles all points accumulated during its effect
	* **Balloons** that weaken the force of gravity on the player

* Advanced Terrain: Additional terrain types with their own advanced physics, generated alongside other terrain types.
	* **Bouncy terrain** that reverses an objects' direction with some added speed
	* **Deep Water** that simulates buoyancy
