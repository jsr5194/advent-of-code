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

	char* token ;
	while(token = strsep(&data, ",")){
		if (token == NULL){
			break;
		}	
	}



	return 0;
}
