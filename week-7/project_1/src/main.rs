use std::io;

fn main() {
println!("Welcome, The following is a list of Job Descriptions.", );
println!("O- Office Administrator
          A- Academic
          L- Lawyer
          T- Teacher");

          let mut job_description = String::new();
          println!("Kindly select your Job Description:");
         io::stdin().read_line(&mut job_description).expect("Failed to read input");
         let job_description = job_description.trim();

         println!("You have successfully selected your job description");
         println!("Kindly input your years of experience.");
         let mut experience = String::new();
         io::stdin().read_line(&mut experience).expect("Failed to read input");
         let experience:i64 = experience.trim().parse().expect("Failed to parse input");

         let office_admin = vec!["Intern", "Administrator","Senior Administrator","Office Manager","Director","CEO"];
         let academic = vec!["Academic","Research Assistant","PhD Candidate","Post Doc Researcher","Senior Lecturer","Dean"];
         let lawyer = vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"];
         let teacher = vec!["Placement Teacher","Classroom Teacher","Senior Teacher","Leadinf Teacher","Deputy Principal","Principal"];


        if job_description == "O" {
            println!("You are an Office Administrator");
            if experience >= 1 && experience <= 2 {
                println!("You are a/an {}", office_admin[0]);
            } 
            else if experience >= 3 && experience <= 5 {
                println!("You are a/an {}", office_admin[1]);
            }
            else if experience >= 5 && experience <= 8 {
                println!("You are a/an {}", office_admin[2]);
           }    
            else if experience >= 8 && experience <= 10 {
                println!("You are a/an {}", office_admin[3]);
          }
          else if experience >= 10 && experience <= 13 {
                println!("You are a/an {}", office_admin[4]);
          }
          else if experience >= 13{
                println!("You are a/an {}", office_admin[5]);
         }
     }
        else if job_description == "A" {
            println!("You are an Academic");
            if experience >= 1 && experience <= 2 {
                println!("You are a/an {}", academic[0]);
            }
            else if experience >= 3 && experience <= 5 {
                println!("You are a/an {}", academic[1]);
            }
            else if experience >= 5 && experience <= 8 {
                println!("You are a/an {}", academic[2]);
            }
            else if experience >= 8 && experience <= 10 {
                println!("You are a/an {}", office_admin[3]);
            }
            else if experience >= 10 && experience <= 13 {
                println!("You are a/an {}", academic[4]);
            }
            else if experience >= 13{
                println!("You are a/an {}", academic[5]);
            }
            }    

        else if job_description == "L" {
            println!("You are a/an Lawyer");
            if experience >= 1 && experience <= 2 {
                println!("You are a/an {}", lawyer[0]);
            }
            else if experience >= 3 && experience <= 5 {
                println!("You are a/an {}", lawyer[1]);
            }
            else if experience >= 5 && experience <= 8 {
                println!("You are a/an {}", lawyer[2]);
            }
            else if experience >= 8 && experience <= 10 {
                println!("You are a/an {}", lawyer[3]);
            }
            else if experience >= 10 && experience <= 13 {
                println!("You are a/an {}", lawyer[4]);
            }
            else if experience >= 13{
                println!("You are a/an {}", academic[5]);
            }
        }    
        else if job_description == "T" {
            println!("You are a Teacher");
            if experience >= 1 && experience <= 2 {
                println!("You are a/an {}", teacher[0]);
            }
            else if experience >= 3 && experience <= 5 {
                println!("You are a/an {}", teacher[1]);
            }
            else if experience >= 5 && experience <= 8 {
                println!("You are a/an {}", teacher[2]);
            }
            else if experience >= 8 && experience <= 10 {
                println!("You are a/an {}", teacher[3]);
            }
            else if experience >= 10 && experience <= 13 {
                println!("You are a/an {}", teacher[4]);
            }
            else if experience >= 13{
                println!("You are a/an {}", teacher[5]);
            }
        }    
        else {
            println!("Invalid Job description");
        }

}
