本文为paper: [Generic Construction of Consensus Algorithms for Benign and Byzantine Faults](https://www.researchgate.net/publication/224165674_Generic_Construction_of_Consensus_Algorithms_for_Benign_and_Byzantine_Faults)的读书笔记, 本文旨在理清论文中所讲的BFT共识. 如果您在阅读过程中有任何意见可以发起ISSUE, 如果喜欢的话可以点击`star`.

## 摘要

>The paper proposes a generic consensus algorithm that highlights the basic and common features of known consensus algorithms. The parameters of the generic algorithm encapsulate the core differences between various consensus algorithms, including leader-based and leader-free algorithms, addressing benign faults, authenticated Byzantine faults and Byzantine faults. This leads to the identification of three classes of consensus algorithms. With the proposed classification, Paxos and PBFT indeed belong to the same class, while FaB Paxos belongs to a different class. Interestingly, the classification allowed us to identify a new Byzantine consensus algorithm that requires n > 4b, where b is the maximum number of Byzantine processes.

本文提出了一种通用的共识算法, 重点介绍了已知共识算法的基本和公共特征. 通用算法的参数封装了各种共识算法之间的核心差异, 包括**有领导者与无领导者的算法, 解决良性故障, 有身份验证的拜占庭容错和拜占庭容错**. 这将用于指导区分3类共识算法的. 基于提出的区分方法, Paxos和PBFT本质上属于同一类, 而FaB Paxos属于另一类. 

## 引言

>Our generic algorithm is based on four parameters: the FLV function, the Selector function, the threshold parameter TD, and the flag FLAG (\* or φ). The functions FLV and Selector are characterized by abstract properties; TD is defined with respect to n (number of processes), f (maximum number of benign faults) and b (maximum number of Byzantine processes). We can prove correctness of the generic consensus algorithm by referring only to the abstract properties of our parameters. The correctness proof of any specific instantiated consensus algorithm consists simply in proving that the instantiations satisfy the abstract properties of the corresponding functions.

