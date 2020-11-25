#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <string.h>


struct path
{
	int numCoordinates;
	struct coordinate *coordinates;
};

struct coordinate
{
	int x;
	int y;
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
	// get file data
	char *data;
	data = readFile("input.txt");

	// split messages
	int maxPaths = 1000;
	int maxCoordinates = 1000000;
	int numPaths = 0;
	char* pathStr;
	struct path *paths[maxPaths];
	while ((pathStr = strsep(&data, "\n"))){
		if (pathStr == '\0'){
			break;
		}

		// prep some variables for tracking path
		struct coordinate coordinates[maxCoordinates];
		int coordinateIndex = 0;
		int numCoordinates = 0; 
		int x = 0;
		int y = 0;

		// split instructions
		char *instr;
		while ((instr = strsep(&pathStr, ","))){
			if (instr == '\0'){
				break;
			}
			int instrLen = sizeof(instr)/sizeof(char);
			char direction = (char)instr[0];
			char tmpDist[instrLen];
			for (int i = 0; i < instrLen-1; i++){
				tmpDist[i] = instr[i+1];
			}
			int distance = atoi(tmpDist);
			
			// plot course
			for (int i = 0; i < distance; i++){
				switch (direction){
					case 82: // R
						x += 1;
						break;
					case 76: // L
						x -= 1;
						break;
					case 85: // U
						y += 1;
						break;
					case 68: // D
						y -= 1;
						break;
					default:
						printf("ERROR: bad direction encountered");
						break;
				}
				struct coordinate point;
				point.x = x;
				point.y = y;

				coordinates[coordinateIndex] = point;
				coordinateIndex++;
				numCoordinates++;
			}
		}

		// create permanent place to store 
		struct path *curPath;
		curPath = malloc(sizeof(struct path));
		curPath->numCoordinates = numCoordinates;
		curPath->coordinates = coordinates;
		paths[numPaths] = curPath;

		numPaths++;
	}

	// free up read in data
	free(data);

//	printf("%d\n", paths[0]->numCoordinates);
//	printf("%d\n", paths[1]->numCoordinates);
//
//	printf("(%d, %d)\n", paths[0]->coordinates[153281].x, paths[0]->coordinates[153281].y);
//	printf("(%d, %d)\n", paths[1]->coordinates[153281].x, paths[1]->coordinates[153281].y);




	return 0;
}