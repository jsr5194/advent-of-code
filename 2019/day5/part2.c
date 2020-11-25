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


struct instruction
{
	int opcode;
	int param1Mode;
	int param2Mode;
	int param3Mode;
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
	tmpInstruction.param1Mode = rawInstruction / 100 % 10; 
	tmpInstruction.param2Mode = rawInstruction / 1000 % 10;
	tmpInstruction.param3Mode = rawInstruction / 10000 % 10;

	return tmpInstruction;
}


int derefParam(int* curTokens, int curParam, int curParamMode)
{
	int retParam;
	switch (curParamMode){
		case POSITION_MODE:
			retParam = curTokens[curParam];
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

int getInput()
{
	char* cBuf;
	cBuf = malloc(2 * sizeof(char));
	printf("Enter an integer > ");
	fgets(cBuf, 2, stdin);
	
	int c;
	c = atoi(cBuf);
	free(cBuf);

	return c;
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
	int numTokens = index ;
	int tokens[numTokens];
	for (int i = 0; i < numTokens; i++){
		tokens[i] = rawTokens[i];
	}

	// start processing 
	int halt = 0;
	for (int cursor = 0; cursor < numTokens; cursor = cursor){
		struct instruction curInstruction = parseInstruction(tokens[cursor]);

		int param1;
		int param2;
		int param3;
		switch(curInstruction.opcode){
			case FUNC_ADD:
				param1 = tokens[cursor+1];
				param2 = tokens[cursor+2];
				param3 = tokens[cursor+3];

				param1 = derefParam(tokens, param1, curInstruction.param1Mode);
				param2 = derefParam(tokens, param2, curInstruction.param2Mode);
				// param3 not run through this as it cannot ever be in a different mode
				tokens[param3] = param1 + param2;

				// increment the cursor by 4 (opcode + 3 params)
				cursor += OPCODE_PLUS_THREE;
				break;

			case FUNC_MULT:
				param1 = tokens[cursor+1];
				param2 = tokens[cursor+2];
				param3 = tokens[cursor+3];

				param1 = derefParam(tokens, param1, curInstruction.param1Mode);
				param2 = derefParam(tokens, param2, curInstruction.param2Mode);
				// param3 not run through this as it cannot ever be in a different mode
				tokens[param3] = param1 * param2;

				// increment the cursor by 4 (opcode + 3 params)
				cursor += OPCODE_PLUS_THREE;
				break;

			case FUNC_INPUT:
				param1 = tokens[cursor+1]; // param1 not run through deref as it cannot ever be in a different mode

				// write input to the specified location
				tokens[param1] = getInput();

				// increment the cursor by 2 (opcode + 1 param)
				cursor += OPCODE_PLUS_ONE;
				break;

			case FUNC_OUTPUT:
				param1 = tokens[cursor+1];
				param1 = derefParam(tokens, param1, curInstruction.param1Mode);

				// send result to output
				printf("Value: %d\n", param1);

				// increment the cursor by 2 (opcode + 1 param)
				cursor += OPCODE_PLUS_ONE;
				break;

			case FUNC_JUMP_TRUE:
				param1 = tokens[cursor+1];
				param2 = tokens[cursor+2];

				param1 = derefParam(tokens, param1, curInstruction.param1Mode);
				param2 = derefParam(tokens, param2, curInstruction.param2Mode);

				if (param1 != 0){
					cursor = param2;
				} else {
					cursor += OPCODE_PLUS_TWO;
				}

				break;

			case FUNC_JUMP_FALSE:
				param1 = tokens[cursor+1];
				param2 = tokens[cursor+2];

				param1 = derefParam(tokens, param1, curInstruction.param1Mode);
				param2 = derefParam(tokens, param2, curInstruction.param2Mode);

				if (param1 == 0){
					cursor = param2;
				} else {
					cursor += OPCODE_PLUS_TWO;
				}

				break;

			case FUNC_LESS_THAN:
				param1 = tokens[cursor+1];
				param2 = tokens[cursor+2];
				param3 = tokens[cursor+3];

				param1 = derefParam(tokens, param1, curInstruction.param1Mode);
				param2 = derefParam(tokens, param2, curInstruction.param2Mode);
				// param3 not run through this as it cannot ever be in a different mode

				if (param1 < param2){
					tokens[param3] = 1;
				} else {
					tokens[param3] = 0;
				}

				cursor += OPCODE_PLUS_THREE;
				break;

			case FUNC_EQUAL:
				param1 = tokens[cursor+1];
				param2 = tokens[cursor+2];
				param3 = tokens[cursor+3];

				param1 = derefParam(tokens, param1, curInstruction.param1Mode);
				param2 = derefParam(tokens, param2, curInstruction.param2Mode);
				// param3 not run through this as it cannot ever be in a different mode

				if (param1 == param2){
					tokens[param3] = 1;
				} else {
					tokens[param3] = 0;
				}

				cursor += OPCODE_PLUS_THREE;			
				break;


			case FUNC_HALT:
				halt = 1;
				break;

			default:
				printf("ERROR: bad opcode (%d) at index %d\n", curInstruction.opcode, cursor);
				exit(1);
				break;
		}

		if (halt == 1){
			break;
		}
	}

	return 0;
}
