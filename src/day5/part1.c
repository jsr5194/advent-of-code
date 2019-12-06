#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

struct instruction
{
	int opcode;
	int param0Mode;
	int param1Mode;
	int param2Mode;
	int rawInstruction;
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
	int modes = rawInstruction / 100;

	// split the modes
	char* strModes;
	int numChars = 3;
	strModes = malloc(numChars * sizeof(char));
	snprintf(strModes, numChars, "%d", modes);
	char* param0Mode = malloc(2 * sizeof(char));
	char* param1Mode = malloc(2 * sizeof(char));
	char* param2Mode = malloc(2 * sizeof(char));
	snprintf(param0Mode, 2, "%c", strModes[2]);
	snprintf(param1Mode, 2, "%c", strModes[1]);
	snprintf(param2Mode, 2, "%c", strModes[0]);
	tmpInstruction.param0Mode = atoi(param0Mode);
	tmpInstruction.param1Mode = atoi(param1Mode);
	tmpInstruction.param2Mode = atoi(param2Mode);

	return tmpInstruction;
}

int main(int argc, char *argv[])
{
	// read in data	
	char *data;
	//data = readFile("/Users/jrittle_adm/src/advent-of-code-2019/src/day2/input.txt");
	data = readFile("input.txt");	

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
	int numTokens = index ;
	int tokens[numTokens];
	for (int i = 0; i < numTokens; i++){
		tokens[i] = rawTokens[i];
	}

	// restore gravity assistant to 1202 program alarm state
	//tokens[1] = 12;
	//tokens[2] = 2;

	// assign some constants
	const int POSITION_MODE = 0;
	const int IMMEDIATE_MODE = 0;
	const int OPCODE_PLUS_THREE = 4;
	const int OPCODE_PLUS_TWO   = 3;
	const int OPCODE_PLUS_ONE   = 2;
	const int OPCODE_PLUS_ZERO  = 1;

	// start processing 
	int halt = 0;
	for (int cursor = 0; cursor < numTokens; cursor = cursor){
		struct instruction curInstruction = parseInstruction(tokens[cursor]);

		printf("Instruction: %d, Opcode: %d, Mode1: %d, Mode2: %d, Mode3: %d\n", curInstruction.rawInstruction, curInstruction.opcode, curInstruction.param0Mode, curInstruction.param1Mode, curInstruction.param2Mode);

		int param1;
		int param2;
		int param3;
		switch(curInstruction.opcode){
			case 1:
				param1 = tokens[cursor+1];
				param2 = tokens[cursor+2];
				param3 = tokens[cursor+3];
				tokens[param3] = tokens[param1] + tokens[param2];

				// increment the cursor by 4 (opcode + 3 params)
				cursor += OPCODE_PLUS_THREE;
				break;
			case 2:
				param1 = tokens[cursor+1];
				param2 = tokens[cursor+2];
				param3 = tokens[cursor+3];
				tokens[param3] = tokens[param1] * tokens[param2];

				// increment the cursor by 4 (opcode + 3 params)
				cursor += OPCODE_PLUS_THREE;
				break;

			case 3:
				// increment the cursor by 2 (opcode + 1 param)
				cursor += OPCODE_PLUS_ONE;
				break;

			case 4:
				// increment the cursor by 2 (opcode + 1 param)
				cursor += OPCODE_PLUS_ONE;
				break;

			case 99:
				halt = 1;
				break;

			default:
				//printf("ERROR: bad opcode (%d) at index %d\n", opcode, cursor);
				break;
		}
		if (halt == 1){
			break;
		}
	}

	printf("Position 0: %d\n", tokens[0]);

	return 0;
}
