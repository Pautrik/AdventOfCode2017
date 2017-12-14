import java.util.Arrays;
import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;

public class d3 {
    public static void main(String[] args) {
        String file = "./input.txt";
        int nrValids = 0;
        try (BufferedReader br = new BufferedReader(new FileReader(file))) {
            for (String rawLine; (rawLine = br.readLine()) != null;) {
                String[] line = rawLine.split(" ");
                Boolean twice = false;
                for (int i = 0; i < line.length - 1 && !twice; i++) {
                    for (int j = i + 1; j < line.length && !twice; j++) {
                        if (line[i].equals(line[j]))
                            twice = true;
                    }
                }

                if (!twice)
                    nrValids++;
            }
        } catch (IOException ioex) {
            ioex.printStackTrace();
        }

        System.out.println(nrValids);
    }
}