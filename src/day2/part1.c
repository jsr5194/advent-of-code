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
	int bufSize = 1024;
	char *data;
	data = readFile("input.txt", bufSize);	


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

	int numTokens = index + 1;
	int tokens[numTokens];
	for (int i = 0; i < numTokens; i++){
		tokens[i] = rawTokens[i];
	}

	for (int i = 0; i < numTokens; i++){
		printf("%d\n", tokens[i]);
	}






	return 0;
}
