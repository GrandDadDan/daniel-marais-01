<h1>Linear Regression Model</h1>
<h1>Introduction To the Problem</h1>

This assignment explores linear regression using Rust and the Burn library (v0.16.0) to predict values based on the function y = 2x + 1. We will generate synthetic data with noise, define a simple AI model, and train it using gradient descent to minimize the Mean Squared Error (MSE) loss function. 
The project covers key machine learning concepts, including data generation, model definition, training, and evaluation. Using Rust Rover IDE and GitHub, we will develop a functional linear regression model and visualize its performance with the textplots crate.

<h2>Installation</h2>

- I downloaded the Rust Rover IDE and Rust via the links that were provided: https://www.rust-lang.org/tools/install and https://www.jetbrains.com/rust/
- I installed the the Rust Rover IDE using the installation prompts and installed the Rust language using the terminal.


<h2>Project Creation</h2>

- I then created a new project in the terminal using cargo to create a new linear regression model.
- I cd'ed into the rust project and opened the project in the Rust Rover IDE.
- Used the dependencies provided and placed them inthe Cargo.toml file, I added no other dependencies as specified or else it results in a zero grading
- Saved the project and proceeded to the next step.
  
<h2>Git Repository Setup</h2>

- Created a new repository from github called daniel-marais-01 as this is the second assignment.
- I then used the terminal within rust to initialize an empty git repository for the project using git init.
- With saved work in the project, I committed the Cargo.toml file seeing that was the only work added so far. Used git commit - m and made a meaningful commit.
- I pushed the saved work to my github repository and continued with the code.

<h1>Approach</h1>

I used documentation from https://www.rust-lang.org/learn. I wanted to get a bit of an understanding of how the language works and saw that it works a little bit differently to
the usual programming languages. It was in a way similar to python programming. I then used YouTube as a secondary resource to understand how to make a linear regression model. 
But was faced with unrelated information which was no help to solving this problem. My last resort was getting help from an AI for the solution which seemed to be working in some way.
I used chatGPT. The problem started to arise when the AI generated code that turned into errors inside the Rust Rover IDE. I kept facing an issue of trying to fix the code using different
imports and going back to chatGPT to fix and relook at the code, only to find that the AI reproduces the same code but presents it differently. I have now hit a wall trying to solve this solution.

I tried a new approach, I tried deconstructing the code received from chatGPT and broke it down into smaller sections and tackled those sections into smaller sections.
I found that I was able to reduce the errors when I deconstructed the code with this method by breaking everything down. However, when the more I did this, the more felt lost as I did not 
understand what I was doing anymore. Entirely, I dont understand the rust language it is completely foreign to me, so this made it harder to understand what the solution should be.
I found that the chatGPT has repeatedly tried to change dependency file based on the problem provided even though the dependencies given were strict. To me this felt like it was intentional. As if the code could
not be solved with the dependencies given.


<h1>Results and Evaluation</h1>

<h2>Rust Website Docummentation</h2>

- https://www.rust-lang.org/learn - I used this to navigate through different documentation on the official Rust website.
- https://doc.rust-lang.org/std/index.html - This link was used to understand how to use imports which was necessary to understand.
- https://doc.rust-lang.org/error_codes/error-index.html - This link was used to understand why I was getting specific errors. When I compiled and ran the project, the program ran into multiple problems, but it gave me
error codes to follow to try and understand why those were errors.

<h2>YouTube Sources</h2>

- https://youtu.be/dn8kjbU2J4U?si=F_9cxwHmm-w8dkCI - This was a video about how to build a rust linear regression model. While there was a lot of fresh information shared in this video, it was not helpful in solving my problem.
- The people in this video broke down what a linear regression model was and explained it in a theory context. They then proceeded to build a linear regression but using a different approach to the dependencies which went against my approach.
- Lastly, all the shorter videos that I watched had different imports to what was given to me.

<h2>ChatGPT AI-Model</h2>

- https://chatgpt.com/share/67bb83ea-52d4-8010-87d4-2900327976dc - This was the first conversation I had with the AI model, where I prompted chatgpt to build a linear regression model using the specifications from the assignment document.
It gave the complete code and I had repeated errors which I could not fix.
- https://chatgpt.com/share/67bb8947-b5b4-8010-8723-006b40750c2e - This was last conversation I had with the AI, I asked it to break everything down so that I could eliminate each error.

<H1>Reflecting on My Learning</H1>

<h2>Help Received From Resources</h2>

I received all the help to build this project using the Rust Documentation, chatGPT, and YouTube resources. I felt that I didnt know anything about this languages so how could know anything, my only solution was to seek help from online sources.
This helped me to gain little understanding of the problem.

<h2>Problem was not Solved</h2>

Despite significant effort, I was unable to successfully complete the implementation of the linear regression model using the Burn library in Rust. The primary obstacles were a limited timeframe and an insufficient depth of knowledge in Rust, particularly in relation to the Burn framework and its tensor operations. Complexities surrounding tensor data structures, optimizer updates, and trait implementations posed significant challenges that could not be fully resolved within the given constraints.

Nevertheless, this experience provided valuable insights into Rust’s application in machine learning, particularly in areas such as tensor manipulation, model architecture, optimization techniques, and data processing. Additionally, it highlighted the importance of understanding Rust’s trait system, debugging strategies, and the constraints imposed by specific dependencies.

Moving forward, a more structured approach—beginning with a stronger foundation in Rust, incremental projects to build familiarity with machine learning libraries, and a thorough study of Burn’s documentation—would be beneficial in addressing similar challenges. Although the project remains incomplete, it served as a valuable learning experience in problem-solving, programming methodologies, and the practical application of machine learning concepts in Rust.


