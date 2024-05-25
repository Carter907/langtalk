pub fn java_text(first_word: &str, other_text: &str) -> String {
    return format!("
```java
public class {} {{
    public static void main(String[] args) {{
        System.out.println(\"{}\");
    }}
}}
```", first_word, other_text);
}
pub fn csharp_text(first_word: &str, other_text: &str) -> String {
    return format!("
```csharp
using System;
class {} {{
    public static void Main(string[] args) {{
        Console.WriteLine(\"{}\");
    }}
}}
```", first_word, other_text);
}