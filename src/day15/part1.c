#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>


// assign some constants
#define FUNC_ADD           1
#define FUNC_MULT          2
#define FUNC_INPUT         3
#define FUNC_OUTPUT        4
#define FUNC_JUMP_TRUE     5
#define FUNC_JUMP_FALSE    6
#define FUNC_LESS_THAN     7
#define FUNC_EQUAL         8
#define FUNC_SET_REL       9
#define FUNC_HALT          99

#define POSITION_MODE      0
#define IMMEDIATE_MODE     1
#define RELATIVE_MODE      2

#define OPCODE_PLUS_THREE  4
#define OPCODE_PLUS_TWO    3
#define OPCODE_PLUS_ONE    2
#define OPCODE_PLUS_ZERO   1

#define NO_INTERRUPT       0
#define INPUT_INTERRUPT    3
#define OUTPUT_INTERRUPT   4
#define HALT_INTERRUPT     99

#define TRUE               1
#define FALSE              0

#define NORTH              1
#define SOUTH              2
#define WEST               3
#define EAST               4

#define POSITION_DEADEND   -3
#define POSITION_START     -2
#define POSITION_UNKNOWN   -1
#define POSITION_WALL      0
#define POSITION_OKAY      1
#define POSITION_O2        2

#define RUN_NORMAL         0
#define RUN_TEST           1
#define RUN_MANUAL         2

#define MAX_X              51
#define MAX_Y              41

struct droid
{
	int x;
	int y;
	int direction;
	int netSteps;
};


struct instruction
{
	int opcode;
	int paramModes[3];
	double params[3];
	int rawInstruction;
};

struct cpuStruct
{
	int programSize;
	double *programBuffer;
	int instructionPointer;
	double input;
	double output;
	int interrupt;
	int relativeBase;
};

char* readFile(char* filename)
{
	// open file
	FILE *f;
	f = fopen(filename, "r");

	// get file details
	struct stat st;
	fstat(fileno(f), &st);

	// store file size
	int fileSize = 0;
	fileSize = st.st_size;

	// read file data
	char *buf = malloc(fileSize);
	fread(buf, fileSize, 1, f);

	// return file data buffer pointer
	return buf;
}

struct instruction parseInstruction(int rawInstruction)
{
	struct instruction tmpInstruction;
	tmpInstruction.rawInstruction = rawInstruction;

	// calculate the opcode
	tmpInstruction.opcode = rawInstruction % 100;

	// separate the modes from the opcode
	tmpInstruction.paramModes[0] = rawInstruction / 100 % 10; 
	tmpInstruction.paramModes[1] = rawInstruction / 1000 % 10;
	tmpInstruction.paramModes[2] = rawInstruction / 10000 % 10;

	return tmpInstruction;
}


double derefParam(struct cpuStruct *curCpu, double curParam, int curParamMode, int isWriteParam)
{
	double retParam;

	switch (curParamMode){

		case POSITION_MODE:
			if (isWriteParam == TRUE){
				retParam = curParam;
			} else{
				retParam = curCpu->programBuffer[(int)curParam];
			}
			
			break;

		case IMMEDIATE_MODE:
			retParam = curParam;
			break;

		case RELATIVE_MODE:
			if (isWriteParam == TRUE){
				retParam = curCpu->relativeBase + curParam;
			} else{
				retParam = curCpu->programBuffer[curCpu->relativeBase + (int)curParam];
			}

			break;

		default:
			printf("ERROR: bad parameter mode encountered");
			break;
	}

	return retParam;
}

void runProgram(struct cpuStruct *cpu)
{
	// start processing 
	while (cpu->instructionPointer < cpu->programSize){
		struct instruction curInstruction = parseInstruction(cpu->programBuffer[cpu->instructionPointer]);

		// swtich on opcode
		switch(curInstruction.opcode){
			case FUNC_ADD:
				curInstruction.params[0] = cpu->programBuffer[cpu->instructionPointer+1];
				curInstruction.params[1] = cpu->programBuffer[cpu->instructionPointer+2];
				curInstruction.params[2] = cpu->programBuffer[cpu->instructionPointer+3];

				curInstruction.params[0] = derefParam(cpu, curInstruction.params[0], curInstruction.paramModes[0], FALSE);
				curInstruction.params[1] = derefParam(cpu, curInstruction.params[1], curInstruction.paramModes[1], FALSE);
				curInstruction.params[2] = derefParam(cpu, curInstruction.params[2], curInstruction.paramModes[2], TRUE);
				
				cpu->programBuffer[(int)curInstruction.params[2]] = curInstruction.params[0] + curInstruction.params[1];

				// increment the cursor by 4 (opcode + 3 params)
				cpu->instructionPointer += OPCODE_PLUS_THREE;
				break;

			case FUNC_MULT:
				curInstruction.params[0] = cpu->programBuffer[cpu->instructionPointer+1];
				curInstruction.params[1] = cpu->programBuffer[cpu->instructionPointer+2];
				curInstruction.params[2] = cpu->programBuffer[cpu->instructionPointer+3];

				curInstruction.params[0] = derefParam(cpu, curInstruction.params[0], curInstruction.paramModes[0], FALSE);
				curInstruction.params[1] = derefParam(cpu, curInstruction.params[1], curInstruction.paramModes[1], FALSE);
				curInstruction.params[2] = derefParam(cpu, curInstruction.params[2], curInstruction.paramModes[2], TRUE);
				
				cpu->programBuffer[(int)curInstruction.params[2]] = curInstruction.params[0] * curInstruction.params[1];

				// increment the cursor by 4 (opcode + 3 params)
				cpu->instructionPointer += OPCODE_PLUS_THREE;
				break;

			case FUNC_INPUT:
				if (cpu->interrupt == NO_INTERRUPT){
					cpu->interrupt = INPUT_INTERRUPT;
					break;
				}
				cpu->interrupt = NO_INTERRUPT;

				curInstruction.params[0] = cpu->programBuffer[cpu->instructionPointer+1];
				curInstruction.params[0] = derefParam(cpu, curInstruction.params[0], curInstruction.paramModes[0], TRUE);

				// write input to the specified location
				cpu->programBuffer[(int)curInstruction.params[0]] = cpu->input;

				// increment the cursor by 2 (opcode + 1 param)
				cpu->instructionPointer += OPCODE_PLUS_ONE;
				break;

			case FUNC_OUTPUT:
				if (cpu->interrupt == NO_INTERRUPT){
					curInstruction.params[0] = cpu->programBuffer[cpu->instructionPointer+1];
					curInstruction.params[0] = derefParam(cpu, curInstruction.params[0], curInstruction.paramModes[0], FALSE);

					// send result to output
					cpu->output = curInstruction.params[0];

					// set the interrupt
					cpu->interrupt = OUTPUT_INTERRUPT;
				}
				// otherwise continue
				else{
					// reset interrupt
					cpu->interrupt = NO_INTERRUPT;

					// increment the cursor by 2 (opcode + 1 param)
					cpu->instructionPointer += OPCODE_PLUS_ONE;
				}
				break;

			case FUNC_JUMP_TRUE:
				curInstruction.params[0] = cpu->programBuffer[cpu->instructionPointer+1];
				curInstruction.params[1] = cpu->programBuffer[cpu->instructionPointer+2];

				curInstruction.params[0] = derefParam(cpu, curInstruction.params[0], curInstruction.paramModes[0], FALSE);
				curInstruction.params[1] = derefParam(cpu, curInstruction.params[1], curInstruction.paramModes[1], FALSE);

				if (curInstruction.params[0] != 0){
					cpu->instructionPointer = curInstruction.params[1];
				} else {
					cpu->instructionPointer += OPCODE_PLUS_TWO;
				}

				break;

			case FUNC_JUMP_FALSE:
				curInstruction.params[0] = cpu->programBuffer[cpu->instructionPointer+1];
				curInstruction.params[1] = cpu->programBuffer[cpu->instructionPointer+2];

				curInstruction.params[0] = derefParam(cpu, curInstruction.params[0], curInstruction.paramModes[0], FALSE);
				curInstruction.params[1] = derefParam(cpu, curInstruction.params[1], curInstruction.paramModes[1], FALSE);

				if (curInstruction.params[0] == 0){
					cpu->instructionPointer = curInstruction.params[1];
				} else {
					cpu->instructionPointer += OPCODE_PLUS_TWO;
				}

				break;

			case FUNC_LESS_THAN:
				curInstruction.params[0] = cpu->programBuffer[cpu->instructionPointer+1];
				curInstruction.params[1] = cpu->programBuffer[cpu->instructionPointer+2];
				curInstruction.params[2] = cpu->programBuffer[cpu->instructionPointer+3];

				curInstruction.params[0] = derefParam(cpu, curInstruction.params[0], curInstruction.paramModes[0], FALSE);
				curInstruction.params[1] = derefParam(cpu, curInstruction.params[1], curInstruction.paramModes[1], FALSE);
				curInstruction.params[2] = derefParam(cpu, curInstruction.params[2], curInstruction.paramModes[2], TRUE);
				
				if (curInstruction.params[0] < curInstruction.params[1]){
					cpu->programBuffer[(int)curInstruction.params[2]] = 1;
				} else {
					cpu->programBuffer[(int)curInstruction.params[2]] = 0;
				}

				cpu->instructionPointer += OPCODE_PLUS_THREE;
				break;

			case FUNC_EQUAL:
				curInstruction.params[0] = cpu->programBuffer[cpu->instructionPointer+1];
				curInstruction.params[1] = cpu->programBuffer[cpu->instructionPointer+2];
				curInstruction.params[2] = cpu->programBuffer[cpu->instructionPointer+3];

				curInstruction.params[0] = derefParam(cpu, curInstruction.params[0], curInstruction.paramModes[0], FALSE);
				curInstruction.params[1] = derefParam(cpu, curInstruction.params[1], curInstruction.paramModes[1], FALSE);
				curInstruction.params[2] = derefParam(cpu, curInstruction.params[2], curInstruction.paramModes[2], TRUE);

				if (curInstruction.params[0] == curInstruction.params[1]){
					cpu->programBuffer[(int)curInstruction.params[2]] = 1;
				} else {
					cpu->programBuffer[(int)curInstruction.params[2]] = 0;
				}

				cpu->instructionPointer += OPCODE_PLUS_THREE;	
				break;

			case FUNC_SET_REL:
				curInstruction.params[0] = cpu->programBuffer[cpu->instructionPointer+1];
				curInstruction.params[0] = derefParam(cpu, curInstruction.params[0], curInstruction.paramModes[0], FALSE);

				cpu->relativeBase += curInstruction.params[0];
				cpu->instructionPointer += OPCODE_PLUS_ONE;
				break;

			case FUNC_HALT:
				cpu->interrupt = HALT_INTERRUPT;
				break;

			default:
				printf("ERROR: bad opcode (%d) at index %d\n", curInstruction.opcode, cpu->instructionPointer);
				exit(1);
				break;
		}

		if (cpu->interrupt != NO_INTERRUPT){
			break;
		}
	}
}

void printMap(int **map, struct droid *repairDroid)
{
	for (int y = 0; y < MAX_Y; y++){
		for (int x = 0; x < MAX_X; x++){
			if (x == repairDroid->x && y == repairDroid->y){
				if (map[repairDroid->y][repairDroid->x] == POSITION_O2){
					printf("2");
				} else{
					printf("D");
				}
			} else{
				switch(map[y][x]){
					case POSITION_UNKNOWN:
						printf(" ");
						break;
					case POSITION_OKAY:
						printf(".");
						break;
					case POSITION_WALL:
						printf("#");
						break;
					case POSITION_START:
						printf("S");
						break;
					case POSITION_DEADEND:
						printf("x");
						break;
					case POSITION_O2:
						printf("2");
						break;
				}
			}
		}
		printf("\n");
	}
}

void checkForDeadend(int** map, struct droid *repairDroid)
{
	// set deadend flags =
	// count th enumber of available spaces to move
	int availableMoves = 0;
	int numDeadends = 0;

	// handle edge cases
	if (repairDroid->x == 0){

	} else if (repairDroid->x == MAX_X-1){

	} else if (repairDroid->y == 0){

	} else if (repairDroid->y == MAX_Y-1){

	} else{
		// count the number of 
		int curNorth = map[repairDroid->y-1][repairDroid->x];
		int curSouth = map[repairDroid->y+1][repairDroid->x];
		int curEast = map[repairDroid->y][repairDroid->x+1];
		int curWest = map[repairDroid->y][repairDroid->x-1];

		if (curNorth == POSITION_OKAY || curNorth == POSITION_UNKNOWN  || curNorth == POSITION_START || curNorth == POSITION_O2){
			availableMoves++;
		} else if (curNorth == POSITION_DEADEND){
			numDeadends++;
		}


		if (curSouth == POSITION_OKAY || curSouth == POSITION_UNKNOWN  || curSouth == POSITION_START || curSouth == POSITION_O2){
			availableMoves++;
		} else if (curSouth == POSITION_DEADEND){
			numDeadends++;
		}


		if (curEast == POSITION_OKAY || curEast == POSITION_UNKNOWN || curEast == POSITION_START || curEast == POSITION_O2){
			availableMoves++;
		} else if (curEast == POSITION_DEADEND){
			numDeadends++;
		}


		if (curWest == POSITION_OKAY || curWest == POSITION_UNKNOWN || curWest == POSITION_START || curWest == POSITION_O2){
			availableMoves++;
		} else if (curWest == POSITION_DEADEND){
			numDeadends++;
		}

		if (availableMoves == 1 && map[repairDroid->y][repairDroid->x] != POSITION_O2){
			map[repairDroid->y][repairDroid->x] = POSITION_DEADEND;
		}
	}
}

void getNextDirection(int** map, struct droid *repairDroid, int runMode)
{
	if (runMode == RUN_NORMAL || runMode == RUN_TEST){

		// allow map to be built one by one
		if (runMode == RUN_TEST){
			getchar();
		}

		int lastX = repairDroid->x;
		int lastY = repairDroid->y;

		switch(repairDroid->direction){
			case NORTH:
				lastY++;
				break;
			case SOUTH:
				lastY--;
				break;
			case EAST:
				lastX--;
				break;
			case WEST:
				lastX++;
				break;
		}

		// look for an unknown location
		if (map[repairDroid->y][repairDroid->x+1] == POSITION_UNKNOWN){
			repairDroid->direction = EAST;
		} else if (map[repairDroid->y][repairDroid->x-1] == POSITION_UNKNOWN){
			repairDroid->direction = WEST;
		} else if (map[repairDroid->y+1][repairDroid->x] == POSITION_UNKNOWN){
			repairDroid->direction = SOUTH;
		} else if (map[repairDroid->y-1][repairDroid->x] == POSITION_UNKNOWN){
			repairDroid->direction = NORTH;
		} 

		// check to see if we've hit a dead end - if so, go to the last good position 
		//
		//     ######
		//         D#
		//     ######
		//
		else if (((map[repairDroid->y][repairDroid->x+1] == POSITION_WALL) || (map[repairDroid->y][repairDroid->x+1] == POSITION_DEADEND)) && 
			     ((map[repairDroid->y-1][repairDroid->x] == POSITION_WALL) || (map[repairDroid->y-1][repairDroid->x] == POSITION_DEADEND)) && 
			     ((map[repairDroid->y+1][repairDroid->x] == POSITION_WALL) || (map[repairDroid->y+1][repairDroid->x] == POSITION_DEADEND))){
			repairDroid->direction = WEST;
			if (map[repairDroid->y][repairDroid->x] != POSITION_O2 && map[repairDroid->y][repairDroid->x] != POSITION_START){
				map[repairDroid->y][repairDroid->x] = POSITION_DEADEND;
				//repairDroid->netSteps--;
			}
		}

		// check to see if we've hit a dead end - if so, go to the last good position 
		//
		//     ######
		//     #D
		//     ######
		//
		else if (((map[repairDroid->y][repairDroid->x-1] == POSITION_WALL) || (map[repairDroid->y][repairDroid->x-1] == POSITION_DEADEND)) &&
		         ((map[repairDroid->y-1][repairDroid->x] == POSITION_WALL) || (map[repairDroid->y-1][repairDroid->x] == POSITION_DEADEND)) && 
			     ((map[repairDroid->y+1][repairDroid->x] == POSITION_WALL) || (map[repairDroid->y+1][repairDroid->x] == POSITION_DEADEND))){
			repairDroid->direction = EAST;
			if (map[repairDroid->y][repairDroid->x] != POSITION_O2 && map[repairDroid->y][repairDroid->x] != POSITION_START){
				map[repairDroid->y][repairDroid->x] = POSITION_DEADEND;
				//repairDroid->netSteps--;
			}
		}

		// check to see if we've hit a dead end - if so, go to the last good position 
		//
		//     ###
		//     #D#
		//     # #
		//
		else if (((map[repairDroid->y-1][repairDroid->x] == POSITION_WALL) || (map[repairDroid->y-1][repairDroid->x] == POSITION_DEADEND)) && 
			     ((map[repairDroid->y][repairDroid->x-1] == POSITION_WALL) || (map[repairDroid->y][repairDroid->x-1] == POSITION_DEADEND)) && 
			     ((map[repairDroid->y][repairDroid->x+1] == POSITION_WALL) || (map[repairDroid->y][repairDroid->x+1] == POSITION_DEADEND))){
			repairDroid->direction = SOUTH;
			if (map[repairDroid->y][repairDroid->x] != POSITION_O2 && map[repairDroid->y][repairDroid->x] != POSITION_START){
				map[repairDroid->y][repairDroid->x] = POSITION_DEADEND;
				//repairDroid->netSteps--;
			}
		}

		// check to see if we've hit a dead end - if so, go to the last good position 
		//
		//     # #
		//     #D#
		//     ###
		//
		else if (((map[repairDroid->y+1][repairDroid->x] == POSITION_WALL) || (map[repairDroid->y+1][repairDroid->x] == POSITION_DEADEND)) && 
			     ((map[repairDroid->y][repairDroid->x-1] == POSITION_WALL) || (map[repairDroid->y][repairDroid->x-1] == POSITION_DEADEND)) && 
			     ((map[repairDroid->y][repairDroid->x+1] == POSITION_WALL) || (map[repairDroid->y][repairDroid->x+1] == POSITION_DEADEND))){
			repairDroid->direction = NORTH;
			if (map[repairDroid->y][repairDroid->x] != POSITION_O2 && map[repairDroid->y][repairDroid->x] != POSITION_START){
				map[repairDroid->y][repairDroid->x] = POSITION_DEADEND;
				//repairDroid->netSteps--;
			}
		}

		// look for anything not a wall or deadend that isn't the last known position
		else if ((repairDroid->y != lastY && repairDroid->x+1 != lastX) && (map[repairDroid->y][repairDroid->x+1] != POSITION_WALL) && (map[repairDroid->y][repairDroid->x+1] != POSITION_DEADEND)){
			repairDroid->direction = EAST;
		} else if ((repairDroid->y != lastY && repairDroid->x-1 != lastX) && (map[repairDroid->y][repairDroid->x-1] != POSITION_WALL)  && (map[repairDroid->y][repairDroid->x-1] != POSITION_DEADEND)){
			repairDroid->direction = WEST;
		} else if ((repairDroid->y+1 != lastY && repairDroid->x != lastX) && (map[repairDroid->y+1][repairDroid->x] != POSITION_WALL) && (map[repairDroid->y+1][repairDroid->x] != POSITION_DEADEND)){
			repairDroid->direction = SOUTH;
		} else if ((repairDroid->y-1 != lastY && repairDroid->x != lastX) && (map[repairDroid->y-1][repairDroid->x] != POSITION_WALL) && (map[repairDroid->y-1][repairDroid->x] != POSITION_DEADEND)){
			repairDroid->direction = NORTH;
		} else{
			printf("wtf\n");
		}

	} else if (runMode == RUN_MANUAL){
		while (TRUE){
			int inputOk = FALSE;
			int bufSize = 32;
			char *inBuf = calloc(bufSize * sizeof(char), bufSize * sizeof(char));
			printf("> ");
			fgets(inBuf, bufSize, stdin);
			switch (inBuf[0]){
				case 'd':
				case '>':
					repairDroid->direction = EAST;
					inputOk = TRUE;
					break;
				case 'w':
				case '^':
					repairDroid->direction = NORTH;
					inputOk = TRUE;
					break;
				case 'a':
				case '<':
					repairDroid->direction = WEST;
					inputOk = TRUE;
					break;
				case 's':
				case 'v':
					repairDroid->direction = SOUTH;
					inputOk = TRUE;
					break;
			}
			free(inBuf);
			if (inputOk == TRUE){
				break;
			} else{
				printf("Try again\n");
			}
		}
	}
}

void restoreDroidLocation(int **map, struct droid* repairDroid)
{
	switch(repairDroid->direction){
		case NORTH:
			repairDroid->y++;
			break;
		case SOUTH:
			repairDroid->y--;
			break;
		case EAST:
			repairDroid->x--;
			break;
		case WEST:
			repairDroid->x++;
			break;
	}
}

int main(int argc, char *argv[])
{
	// read in data	
	char *data;
	data = readFile("input.txt");	
	//data = readFile("test.txt");	

	// separate based on comma
	char* token;
	int maxTokens = 3074;
	double rawTokens[maxTokens];
	int index = 0;
	while((token = strsep(&data, ","))){
		if (token == '\0'){
			break;
		}
		rawTokens[index] = atof(token);
		index++;
	}

	// combine into a properly sized array
	int numTokens = index;
	double tokens[numTokens];
	for (int i = 0; i < numTokens; i++){
		tokens[i] = rawTokens[i];
	}

	// start execution loop
	int halt = FALSE;

	struct cpuStruct cpu;
	cpu.output = 0;  // reset output between permutations
	cpu.programSize = numTokens;
	double programBufferSize = (cpu.programSize * sizeof(double));
	cpu.programBuffer = malloc(programBufferSize);


	// initialize full size to zero
	for (int i = 0; i < programBufferSize; i++){
		cpu.programBuffer[i] = 0;
	}

	// fill in with real program
	for (int i = 0; i < cpu.programSize; i++){
		cpu.programBuffer[i] = tokens[i];
	}
	cpu.instructionPointer = 0;
	cpu.interrupt = NO_INTERRUPT;
	cpu.relativeBase = 0;


	// set up map
	int **map = calloc(MAX_Y * sizeof(int*), MAX_Y * sizeof(int*));
	for (int y = 0; y < MAX_Y; y++){
		int *curRow = calloc(MAX_X * sizeof(int), MAX_X * sizeof(int));
		map[y] = curRow;
		for (int x = 0; x < MAX_X; x++){
			map[y][x] = POSITION_UNKNOWN;
		}
	}

	// initialize
	struct droid repairDroid;
	repairDroid.x = MAX_X/2+1;
	repairDroid.y = MAX_Y/2+1;
	repairDroid.netSteps = 0;
	map[repairDroid.y][repairDroid.x] = POSITION_START;

	printMap(map, &repairDroid);

	// run until a pause is encountered
	// during the first pause, pass in the current phase
	// during all subsequent pauses (just one in this case) pass in the prior output
	while (halt == FALSE){
		switch(cpu.interrupt){
			case INPUT_INTERRUPT:

	
				printMap(map, &repairDroid);

				// for testing
				//getNextDirection(map, &repairDroid, RUN_TEST);
				//getNextDirection(map, &repairDroid, RUN_MANUAL);
				getNextDirection(map, &repairDroid, RUN_NORMAL);

				switch(repairDroid.direction){
					case NORTH:
						repairDroid.y--;
						break;
					case SOUTH:
						repairDroid.y++;
						break;
					case EAST:
						repairDroid.x++;
						break;
					case WEST:
						repairDroid.x--;
						break;
				}

				cpu.input = repairDroid.direction;
				runProgram(&cpu);
				break;

			case OUTPUT_INTERRUPT:
				// increment the step count if output is not a wall
				if (map[repairDroid.y][repairDroid.x] == POSITION_OKAY){
					repairDroid.netSteps--;
				} else if (cpu.output != POSITION_WALL){
					repairDroid.netSteps++;
				}

				checkForDeadend(map, &repairDroid);
				printMap(map, &repairDroid);

				// set the location type in the map
				if (map[repairDroid.y][repairDroid.x] != POSITION_START && map[repairDroid.y][repairDroid.x] != POSITION_DEADEND && map[repairDroid.y][repairDroid.x] != POSITION_O2){
					map[repairDroid.y][repairDroid.x] = cpu.output;
				}
				// else leave it alone

				switch((int)cpu.output){
					case POSITION_WALL:
						// restore the repair droid location
						restoreDroidLocation(map, &repairDroid);
						break;

					case POSITION_O2:
						// restore the repair droid location
						//restoreDroidLocation(map, &repairDroid);

						// print result map
						printMap(map, &repairDroid);

						// print output details
						printf("O2 System Coordinates: (%d,%d)\n", repairDroid.x, repairDroid.y);
						printf("Net Steps: %d\n", repairDroid.netSteps);
						halt = TRUE;

						break;
				}

				runProgram(&cpu);
				break;

			case NO_INTERRUPT:
				runProgram(&cpu);
				break;

			case HALT_INTERRUPT:
				halt = TRUE;
				break;

			default:
				printf("Bad interrupt encountered\n");
				exit(1);
		}
		printf("Steps Taken %d\n", repairDroid.netSteps);
	}


	return 0;
}
