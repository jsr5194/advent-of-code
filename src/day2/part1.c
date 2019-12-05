#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char* readFile(char* filename, int bufSize)
{
	FILE *f;
	char *buf = malloc(bufSize * sizeof(char));
	
	f = fopen(filename, "r");

	while (fgets(buf, bufSize, f) != NULL){
		if (feof(f)){
			break;	
		}
	}
	
	return buf;

}

int main(int argc, char *argv[])
{
	// read in data	
	int bufSize = 1024;
	char *data;
	data = readFile("input.txt", bufSize);	

	// separate based on comma
	char* token;
	int rawTokens[bufSize];
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
	tokens[1] = 12;
	tokens[2] = 2;

	// start processing 
	int halt = 0;
	for (int i = 0; i < numTokens-4; i+=4){
		int opcode = tokens[i];
		int pos1 = tokens[i+1];
		int pos2 = tokens[i+2];
		int pos3 = tokens[i+3];
		switch(opcode){
			case 1:
				tokens[pos3] = tokens[pos1] + tokens[pos2];
				break;
			case 2:
				tokens[pos3] = tokens[pos1] * tokens[pos2];
				break;

			case 99:
				halt = 1;
				break;

			default:
				printf("ERROR: bad opcode (%d) at index %d", opcode, i);
		}
		if (halt == 1){
			break;
		}
	}

	printf("Position 0: %d\n", tokens[0]);

	return 0;
}
