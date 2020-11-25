#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

#define NUM_ROWS    6
#define NUM_COLS    25
#define NUM_LAYERS  100

#define BLACK       0
#define WHITE       1
#define TRANSPARENT 2

struct row{
	int cols[NUM_COLS];
};

struct layer{
	struct row rows[NUM_ROWS]; 
};

struct picture{
	struct layer layers[NUM_LAYERS];
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
	char *data;
	data = readFile("input.txt");

	struct picture curPicture;
	int layerIndex = 0;
	int index = 0;
	while(1){
		// break on the end of data
		if (data[index] == '\0'){
			break;
		}

		// start parsing the input into an image struct
		struct layer curLayer;
		int rowIndex = 0;
		for (rowIndex = 0; rowIndex < NUM_ROWS; rowIndex++){
			int colIndex;
			struct row curRow;
			for (colIndex = 0; colIndex < NUM_COLS; colIndex++){
				curRow.cols[colIndex] = data[index+colIndex]-0x30; // convert ascii int to true int
			}
			curLayer.rows[rowIndex] = curRow;

			index = index + colIndex;
		}
		curPicture.layers[layerIndex] = curLayer;
		layerIndex++;
	}

	int finalPicture[NUM_ROWS][NUM_COLS];
	for (int i = NUM_LAYERS-1; i >= 0; i--){
		for (int j = 0; j < NUM_ROWS; j++){
			for (int k = 0; k < NUM_COLS; k++){
				switch (curPicture.layers[i].rows[j].cols[k]){
					case BLACK:
						finalPicture[j][k] = BLACK;
						break;

					case WHITE:
						finalPicture[j][k] = WHITE;
						break;

					case TRANSPARENT:
						// do nothing
						break;

					default:
						printf("ERROR: bad pixel type\n");
						exit(1);
				}
			}
		}
	}

	for (int i = 0; i < NUM_ROWS; i++){
		for (int j = 0; j < NUM_COLS; j++){
			switch (finalPicture[i][j]){
				case BLACK:
					printf(" ");
					break;

				case WHITE:
					printf("#");
					break;

				default:
					printf("ERROR: bad pixel type\n");
					exit(1);
			}
		}
		printf("\n");
	}



	



	return 0;
}
