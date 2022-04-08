Group Name: Amogus

Group Members: Rafael Wersom (rwers2)
                
Introduction: This project will first construct a game called Hexapawn which is close to chess but using only pawns. Then the game will have a simple AI opponent that gets harder the more you play against it. This project will focus on learning how computers learn; in this case the AI will learn through punishment, as for every loss the computer will be punished and the losing move will be removed from his movepool. Playing for long enough, the AI will play perfectly and never lose.

Rules Hexapawn: As in chess, each pawn may be moved in two different ways: it may be moved one square forward, or it may capture a pawn one square diagonally ahead of it. A pawn may not be moved forward if there is a pawn in the next square. Unlike chess, the first move of a pawn may not advance it by two spaces. A player loses if they have no legal moves or the other player reaches the end of the board with a pawn.

System Overview: The game will mostly be built as a class of it's own with class states holding the data. The AI opponent will be another component and the knowledge of which moves are losing will be stored in a text file which can be accessed and changed by the AI. There will also be an option to play with friends.

Possible Challenges: The hardest parts will be to construct a way for the computer to interact with the data and build the game itself, but overall this will be a fairly simple project and manageable to do it on my own.

References: This idea was inspired by a video from Vsauce2 by the name of "The Game That Learns", in the video, they use matchboxes as a "computer" so I thought translating that into actual code would be fun. link: https://youtu.be/sw7UAZNgGg8