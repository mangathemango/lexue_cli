### Description

The first thing a company does when it is founded is to appoint a CEO. After that, the CEO will start hiring employees, and some employees will leave for other companies. Assuming that every employee in the company (including the CEO) can hire new employees directly and that all employees in the company (including the CEO) can jump ship, the organizational structure of a company after a certain period of time is shown in Figure 1:

![image](https://lexue.bit.edu.cn/pluginfile.php/734238/mod_programming/intro/%E5%85%AC%E5%8F%B8%E9%82%A3%E7%82%B9%E4%BA%8B1.JPG)

Figure 1
VonNeumann is the CEO of the company and employs two people directly, Tanenbaum and Dijkstra, whose positions in the company are determined by the employee's length of service, as shown in the figure from left to right, for example, Tanenbaum's position is higher than Dijkstra's.

When an employee hires a new subordinate, the subordinate has the lowest position among all subordinates hired by the employee. Assuming that VonNeumann hires Shannon, the three subordinates of VonNeumann are Tanenbaum, Dijkstra, and Shannon from highest to lowest position.

When an employee in the company leaves, there are two cases. If he does not employ any subordinates, he is taken out of the company's organizational structure. If he has direct subordinates, then the person in the higher position of his direct subordinates is promoted and fills the missing position. And if that subordinate also has subordinates, the highest-ranking of his subordinates will fill the seat vacated by his promotion. And so on, until an employee who has not yet hired a subordinate is promoted.

Suppose Tanenbaum in Figure 1 jumps ship and leaves, then Stallings will fill its position and Knuth will fill Stallings' position. Figure 2 shows the results of the change: (1) VonNeumann hires Shannon, and (2) Tanenbaum jumps ship.

![image](https://lexue.bit.edu.cn/pluginfile.php/734238/mod_programming/intro/%E5%85%AC%E5%8F%B8%E9%82%A3%E7%82%B9%E4%BA%8B2.JPG)

Figure 2
### Input

The first line of the input is the name of the CEO. All names in the question are between 2 and 20 in length and consist of upper and lower case letters, numbers, and a short line (minus sign). Each name contains at least one uppercase and one lowercase letter.

After the first line, there will be a number of lines, and they consist of the following rules.

- [old employee] hires [new employee]
- fire [old employee]
- print
- end
[Old Employee] is the name of someone who is already working for the company, while [New Employee] is the name of an employee who is about to be hired. The above three rules that make up the content may appear in any order. However, there will be at least one employee (the CEO) in the company and the maximum size of the company will not exceed 1000 people. You should end the program when you see "end".

### Output

For each print command, output the current company structure information according to the following rules:

- Each line contains one person's name
- The first row is the name of the CEO, starting from the first column
- 
Results of the structure in Figure 3，

Figure 3

Will output as in Figure 4

Figure 4


- After each print, a 60-character line of minus is output, without any blank lines in the entire output.
