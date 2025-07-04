//
// Created by Liam Conrad on 7/4/25.
//

#ifndef VALIDATOR_H
#define VALIDATOR_H

#include <iostream>

namespace validator {

    class Validator {
    private:
        char data[32];

    public:
        Validator() : data() {data[0] = '\0'; };
        explicit Validator(const char* _data) : data() { strcpy(data, _data); };
        ~Validator() = default;

        [[nodiscard]] bool vetInput() const {
            bool isValid = true;

            for (int i = 0; i < std::strlen(data); i++) {
                if (data[i] == ' ') continue;
                isValid = std::isdigit(data[i]);
                if (!isValid) {
                    break;
                }
            }

            return isValid;
        }
        void print() const { std::cout << data << std::endl; }
        [[nodiscard]] char* getData() const { return const_cast<char *>(data); }
    }; // class Validator

} // namespace validator

#endif //VALIDATOR_H
