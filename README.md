### Langtalk

mess with your friend on discord with this talk-in-code cli app.

#### Installing
```
git clone https://github.com/Carter907/langtalk
```
then
```
cargo install --path .
```

#### Using

```
langtalk --java --text "Hey You look nice today!"
```
outputs:
```java
public class Hey {
    public static void main(String[] args) {
        System.out.println("you look nice today!");
    }
}
```

#### Supported Languages

- Java
- C#
