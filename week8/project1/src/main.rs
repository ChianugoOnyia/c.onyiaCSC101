use std::io;
fn  main() {
    println!("PUBLIC SERVICE APS LEVEL CHECKER");
    let aps1_2 = ["Intern","Research Assistant","Paralegal","Placement"];
    let aps3_5 = ["Administrator","Research Assistant","Junior Associate","Classroom Teacher"];
    let aps5_8 = ["Senior Administrator","PhD Candidate","Associate","Snr Teacher"];
    let el18_10 = ["Office Manager","Post-Doc Researcher","Senior Associate 1-2","Leading Teacher"];
    let el210_13 = ["Director","Senior Lecturer","Senior Associate 3-4","Deputy Principal"];
    let ses = ["CEO","Dean","Partner","Principal"];

        println!("1 - Office Administrator");
        println!("2 - Academic");
        println!("3 - Lawyer");
        println!("4 - Teacher");
    let mut input1 = String::new();
    println!("Occupation: ");
    io::stdin().read_line(&mut input1).expect("Not valid");
    let code:u32 = input1.trim().parse().expect("Not valid");

     let mut input1 = String::new();
    println!("Number of years worked: ");
    io::stdin().read_line(&mut input1).expect("Not valid");
    let years:u32 = input1.trim().parse().expect("Not valid");


    if code == 1 {
    let m = 0 ;
    if years < 2 {
        
        println!("[{}] rank of Office Administrator with position APS 1-2", aps1_2[m]);
    }
    else if years > 1 && years < 6 {
        
       println!("[{}] rank of Office Administrator with position APS 3-5", aps3_5[m]);
    }
    else if years > 5 && years < 9 {
        
        println!("[{}] rank of Office Administrator with position APS 5-8", aps5_8[m]);
    }
    else if years > 8 && years < 11 {
        
        println!("[{}] rank of Office Administrator with position EL1 8-10", el18_10[m]);
    }
    else if years > 10 && years < 14 {
        
        println!("[{}] rank of Office Administrator with position EL2 10-13",el210_13[m]);
    }
    else if years > 13 {
        
        println!("[{}] rank of Office Administrator with position SES", ses[m]);
    }
  }
  else if code == 2 {
    let m = 1 ;
    if years < 2 {
        
        println!("[{}] rank of Academic with position APS 1-2", aps1_2[m]);
    }
    else if years > 1 && years < 6 {
        
       println!("[{}] rank of Academic with position APS 3-5", aps3_5[m]);
    }
    else if years > 5 && years < 9 {
        
        println!("[{}] rank of Academic with position APS 5-8", aps5_8[m]);
    }
    else if years > 8 && years < 11 {
        
        println!("[{}] rank of Academic with position EL1 8-10", el18_10[m]);
    }
    else if years > 10 && years < 14 {
        
        println!("[{}] rank of Academic with position EL2 10-13",el210_13[m]);
    }
    else if years > 13 {
        
        println!("[{}] rank of Academic with position SES", ses[m]);
    }
  }
  else if code == 3 {
    let m = 2 ;
    if years < 2 {
        
        println!("[{}] rank of Lawyer with position APS 1-2", aps1_2[m]);
    }
    else if years > 1 && years < 6 {
        
       println!("[{}] rank of Lawyer with position APS 3-5", aps3_5[m]);
    }
    else if years > 5 && years < 9 {
        
        println!("[{}] rank of Lawyer with position APS 5-8", aps5_8[m]);
    }
    else if years > 8 && years < 11 {
        
        println!("[{}] rank of Lawyer with position EL1 8-10", el18_10[m]);
    }
    else if years > 10 && years < 14 {
        
        println!("[{}] rank of Lawyer with position EL2 10-13",el210_13[m]);
    }
    else if years > 13 {
        
        println!("[{}] rank of Lawyer with position SES", ses[m]);
    }
  }
   else if code == 4 {
    let m = 3 ;
    if years < 2 {
        
        println!("[{}] rank of Teacher with position APS 1-2", aps1_2[m]);
    }
    else if years > 1 && years < 6 {
        
       println!("[{}] rank of Teacher with position APS 3-5", aps3_5[m]);
    }
    else if years > 5 && years < 9 {
        
        println!("[{}] rank of Teacher with position APS 5-8", aps5_8[m]);
    }
    else if years > 8 && years < 11 {
        
        println!("[{}] rank of Teacher with position EL1 8-10", el18_10[m]);
    }
    else if years > 10 && years < 14 {
        
        println!("[{}] rank of Teacher with position EL2 10-13",el210_13[m]);
    }
    else if years > 13 {
        
        println!("[{}] rank of Teacher with position SES", ses[m]);
    }
  }
  else {
    println!("Not a valid occupation code");
  }

}