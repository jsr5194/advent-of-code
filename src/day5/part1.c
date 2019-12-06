#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

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

int main(int argc, char *argv[])
{
	// read in data	
	char *data;
	data = readFile("/Users/jrittle_adm/src/advent-of-code-2019/src/day2/input.txt");	

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
	tokens[1] = 12;
	tokens[2] = 2;

	// start processing 
	int halt = 0;
	for (int i = 0; i < numTokens-4; i = i){
		int opcode = tokens[i];
		int pos1;
		int pos2;
		int pos3;
		switch(opcode){
			case 1:
				pos1 = tokens[i+1];
				pos2 = tokens[i+2];
				pos3 = tokens[i+3];
				tokens[pos3] = tokens[pos1] + tokens[pos2];
				i+= 4;
				break;
			case 2:
				pos1 = tokens[i+1];
				pos2 = tokens[i+2];
				pos3 = tokens[i+3];
				tokens[pos3] = tokens[pos1] * tokens[pos2];
				i+= 4;
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
