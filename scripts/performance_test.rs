use std::fs;
use std::path::Path;
use std::time::Instant;
use tempfile::TempDir;

fn create_large_test_suite(temp_dir: &Path, file_count: usize, complexity: &str) -> Vec<String> {
    let mut files = Vec::new();
    
    for i in 0..file_count {
        let content = match complexity {
            "realistic" => format!(
                r#"import React from 'react';
import {{ useState, useEffect }} from 'react';
import {{ cn }} from '@/lib/utils';

interface Component{}Props {{
  title: string;
  description?: string;
  variant?: 'primary' | 'secondary' | 'destructive';
  size?: 'sm' | 'md' | 'lg';
  disabled?: boolean;
  loading?: boolean;
  className?: string;
  onClick?: () => void;
}}

export const Component{}: React.FC<Component{}Props> = ({{
  title,
  description,
  variant = 'primary',
  size = 'md',
  disabled = false,
  loading = false,
  className,
  onClick,
}}) => {{
  const [isHovered, setIsHovered] = useState(false);
  const [isFocused, setIsFocused] = useState(false);
  
  const baseClasses = "inline-flex items-center justify-center rounded-md font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";
  
  const variantClasses = {{
    primary: "bg-primary text-primary-foreground hover:bg-primary/90",
    secondary: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
    destructive: "bg-destructive text-destructive-foreground hover:bg-destructive/90",
  }};
  
  const sizeClasses = {{
    sm: "h-9 px-3 text-xs",
    md: "h-10 px-4 py-2 text-sm",
    lg: "h-11 px-8 text-base",
  }};
  
  return (
    <div className="w-full max-w-4xl mx-auto p-6 space-y-6 bg-background">
      <div className="flex items-center justify-between border-b border-border pb-4">
        <h1 className="text-3xl font-bold tracking-tight text-foreground">{{title}}</h1>
        <div className="flex items-center space-x-2">
          <span className="text-sm text-muted-foreground">Component {}</span>
        </div>
      </div>
      
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div className="space-y-2">
          <label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
            Primary Button
          </label>
          <button
            className={{cn(
              baseClasses,
              variantClasses[variant],
              sizeClasses[size],
              loading && "cursor-not-allowed",
              isHovered && "shadow-md transform scale-105",
              isFocused && "ring-2 ring-offset-2",
              className
            )}}
            disabled={{disabled || loading}}
            onClick={{onClick}}
            onMouseEnter={{() => setIsHovered(true)}}
            onMouseLeave={{() => setIsHovered(false)}}
            onFocus={{() => setIsFocused(true)}}
            onBlur={{() => setIsFocused(false)}}
          >
            {{loading && (
              <svg className="mr-2 h-4 w-4 animate-spin" viewBox="0 0 24 24">
                <circle
                  className="opacity-25"
                  cx="12"
                  cy="12"
                  r="10"
                  stroke="currentColor"
                  strokeWidth="4"
                />
                <path
                  className="opacity-75"
                  fill="currentColor"
                  d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                />
              </svg>
            )}}
            {{title}} {{i}}
          </button>
        </div>
        
        <div className="space-y-2">
          <label className="text-sm font-medium leading-none">Description</label>
          <div className="p-4 rounded-lg border border-border bg-muted/50">
            <p className="text-sm text-muted-foreground leading-relaxed">
              {{description || `This is component number {{i}} with various Tailwind classes that need sorting and optimization.`}}
            </p>
          </div>
        </div>
        
        <div className="space-y-2">
          <label className="text-sm font-medium leading-none">Status</label>
          <div className="flex items-center space-x-2">
            <div 
              className={{cn(
                "h-2 w-2 rounded-full",
                disabled ? "bg-destructive" : loading ? "bg-warning animate-pulse" : "bg-success"
              )}}
            />
            <span className="text-xs text-muted-foreground">
              {{disabled ? "Disabled" : loading ? "Loading..." : "Ready"}}
            </span>
          </div>
        </div>
      </div>
      
      <div className="mt-8 space-y-4">
        <div className="flex flex-wrap gap-2">
          <span className="inline-flex items-center rounded-full bg-primary/10 px-2 py-1 text-xs font-medium text-primary ring-1 ring-inset ring-primary/20">
            Component
          </span>
          <span className="inline-flex items-center rounded-full bg-secondary/10 px-2 py-1 text-xs font-medium text-secondary ring-1 ring-inset ring-secondary/20">
            Interactive
          </span>
          <span className="inline-flex items-center rounded-full bg-success/10 px-2 py-1 text-xs font-medium text-success ring-1 ring-inset ring-success/20">
            Accessible
          </span>
        </div>
        
        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6 gap-3">
          {{[...Array(6)].map((_, index) => (
            <div 
              key={{index}}
              className="aspect-square rounded-lg border border-border bg-gradient-to-br from-primary/5 to-secondary/5 p-3 flex items-center justify-center hover:shadow-lg transition-all duration-200 group"
            >
              <div className="text-center space-y-1">
                <div className="h-8 w-8 mx-auto bg-primary/20 rounded-full flex items-center justify-center group-hover:bg-primary/30 transition-colors">
                  <span className="text-xs font-semibold text-primary">{{index + 1}}</span>
                </div>
                <p className="text-xs text-muted-foreground">Item {{index + 1}}</p>
              </div>
            </div>
          ))}}
        </div>
      </div>
    </div>
  );
}};
"#, i, i, i, i
            ),
            _ => format!(
                r#"export const Component{} = () => (
  <div className="p-4 bg-red-500 flex justify-center items-center m-2 text-white">
    Component {}
  </div>
);"#, i, i
            )
        };
        
        let file_path = temp_dir.join(format!("Component{}.tsx", i));
        fs::write(&file_path, content).expect("Failed to write test file");
        files.push(file_path.display().to_string());
    }
    
    files
}

fn main() {
    println!("WindWarden Performance Test Suite");
    println!("=================================");
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Test different scales and complexities
    let test_scenarios = [
        (10, "realistic"),
        (50, "realistic"),
        (100, "realistic"),
        (200, "realistic"),
    ];
    
    for &(file_count, complexity) in &test_scenarios {
        println!("\nTesting {} {} files...", file_count, complexity);
        
        // Create test files
        let start_create = Instant::now();
        let _files = create_large_test_suite(temp_dir.path(), file_count, complexity);
        let create_duration = start_create.elapsed();
        println!("  File creation: {:.2}ms", create_duration.as_secs_f64() * 1000.0);
        
        // Test sequential processing
        let start_seq = Instant::now();
        let output = std::process::Command::new("cargo")
            .args(&["run", "--release", "--", "format", "--mode", "check", "--processing", "sequential", "--stats"])
            .arg(temp_dir.path())
            .output()
            .expect("Failed to run windwarden");
        let seq_duration = start_seq.elapsed();
        
        if output.status.success() {
            println!("  Sequential: {:.2}ms", seq_duration.as_secs_f64() * 1000.0);
        } else {
            println!("  Sequential: FAILED - {}", String::from_utf8_lossy(&output.stderr));
        }
        
        // Test parallel processing
        let start_par = Instant::now();
        let output = std::process::Command::new("cargo")
            .args(&["run", "--release", "--", "format", "--mode", "check", "--processing", "parallel", "--stats"])
            .arg(temp_dir.path())
            .output()
            .expect("Failed to run windwarden");
        let par_duration = start_par.elapsed();
        
        if output.status.success() {
            println!("  Parallel: {:.2}ms", par_duration.as_secs_f64() * 1000.0);
            
            if seq_duration.as_millis() > 0 {
                let speedup = seq_duration.as_secs_f64() / par_duration.as_secs_f64();
                println!("  Speedup: {:.2}x", speedup);
            }
            
            // Parse output for statistics
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Some(stats_start) = stdout.find("Statistics:") {
                let stats_section = &stdout[stats_start..];
                for line in stats_section.lines().take(10) {
                    if line.contains("Files/sec:") || line.contains("Duration:") || line.contains("Success rate:") {
                        println!("  {}", line.trim());
                    }
                }
            }
        } else {
            println!("  Parallel: FAILED - {}", String::from_utf8_lossy(&output.stderr));
        }
        
        // Clean up for next iteration
        fs::remove_dir_all(temp_dir.path().join("Component*")).unwrap_or(());
        
        // Recreate empty directory
        fs::create_dir_all(temp_dir.path()).unwrap_or(());
    }
    
    println!("\nPerformance test complete!");
}