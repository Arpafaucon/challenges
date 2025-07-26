#include <cstdio>
#include <string>
#include <vector>
#include <iostream>
#include <map>
#include <sstream>



std::vector<std::string> split(std::string& s, const std::string& delimiter) {
    std::vector<std::string> tokens;
    size_t pos = 0;
    std::string token;
    while ((pos = s.find(delimiter)) != std::string::npos) {
        token = s.substr(0, pos);
        tokens.push_back(token);
        s.erase(0, pos + delimiter.length());
    }
    tokens.push_back(s);

    return tokens;
}


std::vector<std::string> process(std::vector<std::string> const& logs)
{
    std::map<int, std::vector<std::string>> clients;
    int clientCounter = 1;

    std::vector<std::string> answers;


    for (auto const& log: logs)
    {
        printf("%s\n", log.c_str());

        if (log[0] == 'Q')
        {
            std::string pattern = log.substr(3);

            // TODO split patterns
            clients[clientCounter] = split(pattern, " ");

            std::ostringstream answer;
            answer << "ACK: " << log.substr(3) << "; ID=" << clientCounter;
            clientCounter++;
            answers.push_back(answer.str());
        }

        if (log[0] == 'L')
        {

            // for (auto const& client: clients)
            // {
            //     for (auto const& pattern: client.second)
            //     {
            //         if (s.find(delimiter)) != std::string::npos)
            //         {

            //         }
            //     }
            // }

            printf("Log\n");
        }

    }

    for (auto const& answer : answers)
    {
        printf("%s\n", answer.c_str());
    }

    return {};
};





int main(int argc, char* argv[])
{
    std::vector<std::string> logs = {
        "Q: failed",
        "L: failed to compile",
        "Q: crash compilation",
        "L: compiled meshes",
        "L: compilation failed system crash"
    };

    process(logs);

    return 0;
}



// messages = [
    // "Q: failed",
    // "L: failed to compile",
    // "Q: crash compilation",
    // "L: compiled meshes"
    // "L: compilation failed system crash"
// ]


// expected = [
//     "ACK: failed; ID=1",
//     "M: failed to compile; Q=1"
//     "ACK: crash compilation; ID=2",
//     # log line does not match anything
//     "M: compilation failed system crash; Q=1,2" # match 2 IDs
// ]
