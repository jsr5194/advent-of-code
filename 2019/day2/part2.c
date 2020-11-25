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

	for (int noun = 0; noun < 100; noun++){
		for (int verb = 0; verb < 100; verb++){
			int memory[numTokens];
			for (int i = 0; i < numTokens; i++){
				memory[i] = tokens[i];
			}

			// restore gravity assistant to 1202 program alarm state
			memory[1] = noun;
			memory[2] = verb;

			// start processing 
			int halt = 0;
			for (int i = 0; i < numTokens-4; i+=4){
				int instruction = memory[i];
				int param1 = memory[i+1];
				int param2 = memory[i+2];
				int param3 = memory[i+3];
				switch(instruction){
					case 1:
						memory[param3] = memory[param1] + memory[param2];
						break;
					case 2:
						memory[param3] = memory[param1] * memory[param2];
						break;

					case 99:
						halt = 1;
						break;

					default:
						printf("ERROR: bad instruction (%d) at index %d", instruction, i);
				}
				if (halt == 1){
					break;
				}
			}

			if (memory[0] == 19690720){
				printf("noun: %d\n", noun);
				printf("verb: %d\n", verb);
				printf("Answer: %d\n", 100*noun+verb);
			}
		}
	}

	return 0;
}
