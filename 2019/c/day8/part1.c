#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

#define NUM_ROWS    6
#define NUM_COLS    25
#define NUM_LAYERS  100

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

	// determine layer with fewest 0 digits
	int answerLayer = 0;
	int lowest0Digits = 1000;
	for (int i = 0; i < NUM_LAYERS; i++){
		int cur0Digits = 0;
		for (int j = 0; j < NUM_ROWS; j++){
			for (int k = 0; k < NUM_COLS; k++){
				if (curPicture.layers[i].rows[j].cols[k] == 0x0){
					cur0Digits++;
				}
			}
		}
		if (cur0Digits < lowest0Digits){
			lowest0Digits = cur0Digits;
			answerLayer = i;
		}
	}

	// figure out answer
	int num1Digits = 0;
	int num2Digits = 0;
	for (int i = 0; i < NUM_ROWS; i++){
		for (int j = 0; j < NUM_COLS; j++){
			switch(curPicture.layers[answerLayer].rows[i].cols[j]){
				case 0x1:
					num1Digits++;
					break;

				case 0x2:
					num2Digits++;
					break;
			}
		}
	}

	int answer = num1Digits * num2Digits;
	printf("Answer: %d\n", answer);




	return 0;
}
