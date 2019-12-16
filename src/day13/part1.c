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

#define ID_EMPTY           0
#define ID_WALL            1
#define ID_BLOCK           2
#define ID_HORIZ_PADDLE    3
#define ID_BALL            4


struct tile
{
	int x;
	int y;
	int id;
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

//		printf("INSPTR: %d \nPROG: ", cpu->instructionPointer);
//		for (int i = 0; i < (10*cpu->programSize); i++){
//			printf("%0.0lf ", cpu->programBuffer[i]);
//		}
//		printf("\n\n");

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

	// build game screen
	int maxX = 100;
	int maxY = 100;
	struct tile grid[maxY][maxX];
	for (int y = 0; y < maxY; y++){
		for (int x = 0; x < maxX; x++){
			struct tile curTile;
			curTile.x = x;
			curTile.y = y;
			curTile.id = ID_EMPTY;
			grid[y][x] = curTile;
		}
	}

	// initialize
	int outputCounter = 0;
	int curX = 0;
	int curY = 0;

	// run until a pause is encountered
	// during the first pause, pass in the current phase
	// during all subsequent pauses (just one in this case) pass in the prior output
	while (halt == FALSE){
		switch(cpu.interrupt){
			case INPUT_INTERRUPT:
				runProgram(&cpu);
				break;

			case OUTPUT_INTERRUPT:
				switch (outputCounter){
					case 0:
						curX = cpu.output;
						outputCounter = 1;
						break;

					case 1:
						curY = cpu.output;
						outputCounter = 2;
						break;

					case 2:
						grid[curY][curX].id = cpu.output;
						outputCounter = 0;
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
	}

	int numBlocks = 0;
	for (int y = 0; y < maxY; y++){
		for (int x = 0; x < maxX; x++){
			if (grid[y][x].id == ID_BLOCK){
				numBlocks++;
			}
		}
	}

	printf("Answer: %d\n", numBlocks);


	return 0;
}
