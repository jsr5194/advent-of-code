#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <string.h>

struct planet {
	char* id;
	struct planet *orbiters;
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

int main(int argc, char *argv[])
{
	// get raw data
	char *data;
	data = readFile("input.txt");

	// define some vars
	const int MAX_PLANETS = 8192;
	//structure planet *planets[MAX_PLANETS];
	char *orbitees[MAX_PLANETS];
	char *orbiters[MAX_PLANETS];
	int planetIndex = 0;
	
	// separate based on newline
	char* lineToken;
	char* planetToken;
	int lineIndex = 0;
	while((lineToken = strsep(&data, "\n"))){
		if (lineToken == '\0'){
			break;
		}

		// separate based on ')' to get a list of planet names
		int orbiter = 0;
		while((planetToken = strsep(&lineToken, ")"))){
			if (planetToken == '\0'){
				break;
			}



			if (orbiter == 1){
				orbiters[lineIndex] = planetToken;
				printf("%s\n", orbiters[lineIndex]);
			} else{
				orbitees[lineIndex] = planetToken;
				orbiter = 1;
				printf("%s)", orbitees[lineIndex]);
			}


		}
		lineIndex++;
	}

	
//	for (int i = 0; i < lineIndex; i++){
//		printf("%s)%s\n", orbitees[lineIndex], orbiters[lineIndex]);
//	}

	return 0;
}