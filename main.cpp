#include <iostream>
#include "validator.h"

using namespace std;
using namespace validator;


int main() {
    Validator v{};

    cout << "Enter a string to be validated: " << endl;

    cin.getline(v.getData(), 32);

    switch (const Result v_result = v.isValid()) {
        case VALID:
            cout << "Input is valid." << endl;
            break;
        case INVALID:
            cout << "Input is not valid." << endl;
            break;
        case ERR_BAD_INPUT:
            cout << "Input contains invalid characters." << endl;
            break;
        // Add more cases as needed
    }

    return 0;
}
