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
#define FUNC_HALT          99

#define POSITION_MODE      0
#define IMMEDIATE_MODE     1

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


struct instruction
{
	int opcode;
	int paramModes[3];
	int params[3];
	int rawInstruction;
};

struct phaseGroupStruct 
{
	int phases[5];
	char *rawPhaseGroup;
};

struct cpuStruct
{
	int programSize;
	int *programBuffer;
	int instructionPointer;
	int input;
	int output;
	int interrupt;
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


int derefParam(int* programBuffer, int curParam, int curParamMode)
{
	int retParam;
	switch (curParamMode){
		case POSITION_MODE:
			retParam = programBuffer[curParam];
			break;

		case IMMEDIATE_MODE:
			retParam = curParam;
			break;

		default:
			printf("ERROR: bad parameter mode encountered");
			break;
	}

	return retParam;
}

// function to generate permutation
// https://rosettacode.org/wiki/Permutations#version_1
struct phaseGroupStruct getPermutationEntry(struct phaseGroupStruct initialPhaseGroup, int desiredIndex){

	//it calculates an array's length
	int x;
	for (x = 0; initialPhaseGroup.rawPhaseGroup[x] != '\0'; x++);

	//buble sort the array
	int f, v, m;
	for(f=0; f < x; f++) {
		for(v = x-1; v > f; v-- ) {
			if (initialPhaseGroup.rawPhaseGroup[v-1] > initialPhaseGroup.rawPhaseGroup[v]) {
				m=initialPhaseGroup.rawPhaseGroup[v-1];
				initialPhaseGroup.rawPhaseGroup[v-1]=initialPhaseGroup.rawPhaseGroup[v];
				initialPhaseGroup.rawPhaseGroup[v]=m;
			}
		}
	}

	//it calculates a factorial to stop the algorithm
	char a[x];
	int k=0;
	int fact=k+1;
	while (k!=x) {
		a[k]=initialPhaseGroup.rawPhaseGroup[k];
		k++;
		fact = k*fact;
	}
	a[k]='\0';

	// build return phase group struct
	struct phaseGroupStruct retPhaseGroup;

	//Main part: here we permutate
	int i, j;
	int y=0;
	char c;
	while (y != fact-1) {
		i=x-2;
		while(a[i] > a[i+1] ) i--;
		j=x-1;
		while(a[j] < a[i] ) j--;
		c=a[j];
		a[j]=a[i];
		a[i]=c;
		i++;
		for (j = x-1; j > i; i++, j--) {
			c = a[i];
			a[i] = a[j];
			a[j] = c;
		}

		if (y == desiredIndex){
			for (int len = 0; len < x; len++){
				retPhaseGroup.phases[len] = a[len]-0x30; // -0x30 to convert into integer
			}
			break;
		}
		y++;
	}
	return retPhaseGroup;
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

				curInstruction.params[0] = derefParam(cpu->programBuffer, curInstruction.params[0], curInstruction.paramModes[0]);
				curInstruction.params[1] = derefParam(cpu->programBuffer, curInstruction.params[1], curInstruction.paramModes[1]);
				// param3 not run through this as it cannot ever be in a different mode
				cpu->programBuffer[curInstruction.params[2]] = curInstruction.params[0] + curInstruction.params[1];

				// increment the cursor by 4 (opcode + 3 params)
				cpu->instructionPointer += OPCODE_PLUS_THREE;
				break;

			case FUNC_MULT:
				curInstruction.params[0] = cpu->programBuffer[cpu->instructionPointer+1];
				curInstruction.params[1] = cpu->programBuffer[cpu->instructionPointer+2];
				curInstruction.params[2] = cpu->programBuffer[cpu->instructionPointer+3];

				curInstruction.params[0] = derefParam(cpu->programBuffer, curInstruction.params[0], curInstruction.paramModes[0]);
				curInstruction.params[1] = derefParam(cpu->programBuffer, curInstruction.params[1], curInstruction.paramModes[1]);
				// param3 not run through this as it cannot ever be in a different mode
				cpu->programBuffer[curInstruction.params[2]] = curInstruction.params[0] * curInstruction.params[1];

				// increment the cursor by 4 (opcode + 3 params)
				cpu->instructionPointer += OPCODE_PLUS_THREE;
				break;

			case FUNC_INPUT:
				if (cpu->interrupt == NO_INTERRUPT){
					cpu->interrupt = INPUT_INTERRUPT;
					break;
				}
				cpu->interrupt = NO_INTERRUPT;

				curInstruction.params[0] = cpu->programBuffer[cpu->instructionPointer+1]; // param1 not run through deref as it cannot ever be in a different mode

				// write input to the specified location
				cpu->programBuffer[curInstruction.params[0]] = cpu->input;

				// increment the cursor by 2 (opcode + 1 param)
				cpu->instructionPointer += OPCODE_PLUS_ONE;
				break;

			case FUNC_OUTPUT:
				if (cpu->interrupt == NO_INTERRUPT){
					curInstruction.params[0] = cpu->programBuffer[cpu->instructionPointer+1];
					curInstruction.params[0] = derefParam(cpu->programBuffer, curInstruction.params[0], curInstruction.paramModes[0]);

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

				curInstruction.params[0] = derefParam(cpu->programBuffer, curInstruction.params[0], curInstruction.paramModes[0]);
				curInstruction.params[1] = derefParam(cpu->programBuffer, curInstruction.params[1], curInstruction.paramModes[1]);

				if (curInstruction.params[0] != 0){
					cpu->instructionPointer = curInstruction.params[1];
				} else {
					cpu->instructionPointer += OPCODE_PLUS_TWO;
				}

				break;

			case FUNC_JUMP_FALSE:
				curInstruction.params[0] = cpu->programBuffer[cpu->instructionPointer+1];
				curInstruction.params[1] = cpu->programBuffer[cpu->instructionPointer+2];

				curInstruction.params[0] = derefParam(cpu->programBuffer, curInstruction.params[0], curInstruction.paramModes[0]);
				curInstruction.params[1] = derefParam(cpu->programBuffer, curInstruction.params[1], curInstruction.paramModes[1]);

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

				curInstruction.params[0] = derefParam(cpu->programBuffer, curInstruction.params[0], curInstruction.paramModes[0]);
				curInstruction.params[1] = derefParam(cpu->programBuffer, curInstruction.params[1], curInstruction.paramModes[1]);
				// param3 not run through this as it cannot ever be in a different mode

				if (curInstruction.params[0] < curInstruction.params[1]){
					cpu->programBuffer[curInstruction.params[2]] = 1;
				} else {
					cpu->programBuffer[curInstruction.params[2]] = 0;
				}

				cpu->instructionPointer += OPCODE_PLUS_THREE;
				break;

			case FUNC_EQUAL:
				curInstruction.params[0] = cpu->programBuffer[cpu->instructionPointer+1];
				curInstruction.params[1] = cpu->programBuffer[cpu->instructionPointer+2];
				curInstruction.params[2] = cpu->programBuffer[cpu->instructionPointer+3];

				curInstruction.params[0] = derefParam(cpu->programBuffer, curInstruction.params[0], curInstruction.paramModes[0]);
				curInstruction.params[1] = derefParam(cpu->programBuffer, curInstruction.params[1], curInstruction.paramModes[1]);
				// param3 not run through this as it cannot ever be in a different mode

				if (curInstruction.params[0] == curInstruction.params[1]){
					cpu->programBuffer[curInstruction.params[2]] = 1;
				} else {
					cpu->programBuffer[curInstruction.params[2]] = 0;
				}

				cpu->instructionPointer += OPCODE_PLUS_THREE;	
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
	int maxTokens = 1024;
	int rawTokens[maxTokens];
	int index = 0;
	while((token = strsep(&data, ","))){
		if (token == '\0'){
			break;
		}
		rawTokens[index] = atoi(token);
		index++;
	}

	// combine into a properly sized array
	int numTokens = index;
	int tokens[numTokens];
	for (int i = 0; i < numTokens; i++){
		tokens[i] = rawTokens[i];
	}





	// setup an array to hold all possible phase groups
	char *initialRawPhaseGroup = "56789";
	int numPhaseGroups = 120; // 5 * 4 * 3 * 2 * 1
	struct phaseGroupStruct phaseGroups[numPhaseGroups];

	// build initial phase group
	struct phaseGroupStruct initialPhaseGroup;
	initialPhaseGroup.rawPhaseGroup = initialRawPhaseGroup;
	initialPhaseGroup.phases[0] = 5;
	initialPhaseGroup.phases[1] = 6;
	initialPhaseGroup.phases[2] = 7;
	initialPhaseGroup.phases[3] = 8;
	initialPhaseGroup.phases[4] = 9;

	// add all possible phase groups 
	phaseGroups[0] = initialPhaseGroup;
	for (int i = 0; i < (numPhaseGroups-1); i++){ // minus one comes from adding the initial phase group
		phaseGroups[i+1] = getPermutationEntry(initialPhaseGroup, i);
	}




// //TESTING
//	// setup an array to hold all possible phase groups
//	char *initialRawPhaseGroup = "98765";
//	int numPhaseGroups = 1; // 5 * 4 * 3 * 2 * 1
//	struct phaseGroupStruct phaseGroups[numPhaseGroups];
//
//	// build initial phase group
//	struct phaseGroupStruct initialPhaseGroup;
//	initialPhaseGroup.rawPhaseGroup = initialRawPhaseGroup;
//	initialPhaseGroup.phases[0] = 9;
//	initialPhaseGroup.phases[1] = 7;
//	initialPhaseGroup.phases[2] = 8;
//	initialPhaseGroup.phases[3] = 5;
//	initialPhaseGroup.phases[4] = 6;
//	phaseGroups[0] = initialPhaseGroup;





	int numPhases = 5;
	int largestOutput = 0;
	for (int phaseGroupIndex = 0; phaseGroupIndex < numPhaseGroups; phaseGroupIndex++){

		// create a group of cpu structs
		struct cpuStruct cpus[numPhases];
		for (int i = 0; i < numPhases; i++){
			struct cpuStruct curCpu;
			curCpu.output = 0;  // reset output between permutations
			curCpu.programSize = numTokens;
			curCpu.programBuffer = malloc(numTokens * sizeof(int));
			for (int i = 0; i < curCpu.programSize; i++){
				curCpu.programBuffer[i] = tokens[i];
			}
			curCpu.instructionPointer = 0;
			curCpu.interrupt = NO_INTERRUPT;
			cpus[i] = curCpu;
		}





		// start execution loop
		int firstRun = TRUE;
		int phaseIndex = 0;
		int halt = FALSE;
		int cpuPhaseSettings[numPhases];

		for (int i = 0; i < numPhases; i++){
			cpuPhaseSettings[i] = TRUE;
		}

		// run until a pause is encountered
		// during the first pause, pass in the current phase
		// during all subsequent pauses (just one in this case) pass in the prior output
		while (halt == FALSE){
			switch(cpus[phaseIndex].interrupt){
				case INPUT_INTERRUPT:
					if (cpuPhaseSettings[phaseIndex] == TRUE){
						cpus[phaseIndex].input = phaseGroups[phaseGroupIndex].phases[phaseIndex];
						cpuPhaseSettings[phaseIndex] = FALSE;
					} else{
						if (firstRun == TRUE){
							cpus[phaseIndex].input = 0;
							firstRun = FALSE;
						} else{
							if (phaseIndex == 0){
								cpus[phaseIndex].input = cpus[numPhases-1].output;
							} else{
								cpus[phaseIndex].input = cpus[phaseIndex-1].output;
							}
						}
					}
					runProgram(&cpus[phaseIndex]);
					break;

				case OUTPUT_INTERRUPT:
					if (phaseIndex == (numPhases-1)){
						phaseIndex = 0;
						runProgram(&cpus[phaseIndex]);
					} else{
						phaseIndex++;
						runProgram(&cpus[phaseIndex]);
					}

					break;

				case NO_INTERRUPT:
					runProgram(&cpus[phaseIndex]);
					break;

				case HALT_INTERRUPT:
					halt = TRUE;
					break;

				default:
					printf("Bad interrupt encountered\n");
					exit(1);
			}
		}
		
		// keep track of the largest result
		if (cpus[numPhases-1].output > largestOutput){
			largestOutput = cpus[numPhases-1].output;
		}
	}

	printf("Answer: %d\n", largestOutput);

	return 0;
}
