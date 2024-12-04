# Daytona Rust/Actix-Web Sample  

This repository contains a sample project showcasing how to build a web application using **Rust** and **Actix-Web**. It is designed as a template for Daytona samples and adheres to the MIT License.  

---

## 📝 Overview  

This sample demonstrates the following:  
- Building a web server using **Actix-Web**  
- Structuring a Rust web application project  
- Using Daytona for standardized development workflows  

---

## 🚀 Getting Started  

### Prerequisites  

Ensure you have the following tools installed:  
- Rust (stable version) – [Install Rust](https://www.rust-lang.org/tools/install)  
- Daytona – Follow the [Daytona Installation Guide](https://www.daytona.io/docs/installation/installation/)  

### Open Using Daytona  

1. **Install Daytona**:  
   Follow the Daytona installation guide linked above.  

2. **Create the Workspace**:  
   Use Daytona to clone and initialize the sample project:  
   ```bash  
   daytona create https://github.com/ayimdomnic/daytona-rs-sample  
   ```  

3. **Navigate to the Project Directory**:  
   ```bash  
   cd daytona-rs-sample  
   ```  

4. **Run the Application**:  
   Use Cargo to start the server:  
   ```bash  
   cargo run  
   ```  

5. **Access the Application**:  
   Open your browser and navigate to `http://localhost:8080` (default).  

---

## ✨ Features  

This sample includes the following features:  
- **Basic Web Server**: A simple Actix-Web server to handle HTTP requests.  
- **Modular Architecture**: Clean separation of concerns with examples of routing and middleware.  
- **Development Environment**: Preconfigured setup compatible with Daytona's devcontainers.  

---

## 🛠 Customization  

To adapt this sample to your needs:  
1. Modify the `src/main.rs` file to add your routes and handlers.  
2. Update `Cargo.toml` to include additional dependencies.  
3. Test your changes using the provided scripts or add your own test suite.  

---

## 🤝 Contributing  

We welcome contributions! To propose a change:  
1. Fork the repository and create a feature branch.  
2. Make your changes and commit them with clear messages.  
3. Open a pull request for review.  

---

## 📄 License  

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.  

---

## 🔗 Resources  

- [Actix-Web Documentation](https://actix.rs/)  
- [Rust Programming Language](https://www.rust-lang.org/)  
- [Daytona Documentation](https://www.daytona.io/docs/)  