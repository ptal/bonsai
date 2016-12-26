package bonsai.cp.core;

import java.util.*;
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;
import bonsai.chococubes.core.*;
import bonsai.chococubes.choco.*;
import bonsai.chococubes.sugarcubes.*;

public class OneSolution implements Executable
{
  private channel single_time FlatLattice<Consistent> consistent = bot;

  public proc execute() {
    loop {
      when consistent |= Consistent.True {
        exit FoundSolution;
      }
      pause;
    }
  }
}
