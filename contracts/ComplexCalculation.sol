pragma solidity ^0.5.1;
pragma experimental ABIEncoderV2;

contract ComplexCalculation {

    string equation;
    address[] listOfProviders;
    mapping(address => bool) userEquationExists;
    mapping(address => bool) providerSubmittedSolution;
    mapping(address => string) public currentUserEquations;
    mapping(address => string[]) public proposedResultsByProviders;
    mapping(address => string) public finalResults;
    mapping(string => uint) public consistencyScore;
    
    
    function getProposedResults(address _requester) public returns(string[] memory propsoedResults_) {
        // for (uint z=0; z<proposedResultsByProviders[_requester].length; z++) {
            
        // }
        return proposedResultsByProviders[_requester];
    }

    event SolveThisEquation(address requestingAddress, string requestedEquation);
    
    function requestCalculation(string memory _equation) public {
        
        emit SolveThisEquation(msg.sender, _equation);
        userEquationExists[msg.sender] = true;
        currentUserEquations[msg.sender] = _equation;
    }
    
    function submitCalculation(address _requester, string memory _result) public returns (bool _submitStatus) {
        require(providerSubmittedSolution[msg.sender]==false);
        providerSubmittedSolution[msg.sender] = true;
        listOfProviders.push(msg.sender);
        proposedResultsByProviders[_requester].push(_result);
        return true;
    }
    
    function checkConsistentValue(string[] memory _proposedResults) internal 
    returns(string memory consistentValue_, uint consistencyCount_, uint nodeCount_) {
        
        uint max;
        uint total = _proposedResults.length;
        string memory consistentValue;
        
        for(uint i=0; i<_proposedResults.length; i++) {
            // uint score = consistencyScore[_proposedResults[i]];
            // consistencyScore[_proposedResults[i]] = score + 1;
            consistencyScore[_proposedResults[i]]++;
            // if(_proposedResults[i])
            if (consistencyScore[_proposedResults[i]] > max){
                max = consistencyScore[_proposedResults[i]];
                consistentValue = _proposedResults[i];
            }
        }
        consistencyScore[consistentValue] = 0;
        return (consistentValue, max, total);
        // return ("consistentValue",1);
    }
    
    function publishCalculation() public returns (string memory finalResult_, uint finalConsistencyCount_, uint totalNodeCount_) {
        if (userEquationExists[msg.sender]){
            // LOGIC TO CHECK THE MOST CONSISTENT SOLUTION AND PUBLISH IT TO THE REQUESTED PARTY
            string memory finalConsistentValue;
            uint finalConsistencyCount;
            uint nodeCount;
            (finalConsistentValue,finalConsistencyCount,nodeCount) = checkConsistentValue(proposedResultsByProviders[msg.sender]);
            
            finalResults[msg.sender] = finalConsistentValue;
            delete proposedResultsByProviders[msg.sender];
            for (uint d = 0; d<listOfProviders.length; d++) {
                delete providerSubmittedSolution[listOfProviders[d]];
            }
            delete currentUserEquations[msg.sender];
            return (finalResults[msg.sender],finalConsistencyCount,nodeCount);
        }
    }
}
