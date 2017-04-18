# mod strings: Keep human readable text out of your code

This exists to experiment with keeping the constant strings and
other build resources out of the codebase for locale targeting 
among other things.

Two examples exist: 

- `*.in`: Which are to be included as a `!include_str()` directive. In the future, these may 
   be preprocessed by build.rs for translation, spellchecking, etc.
   
- `*.rsrc.xml`: Idea to keep in mind for a more general build resource a la android. With 
   serde_xml it should be easy to create a build manifest that is available as a library
   to all consumers.

**Current Guidance:** Use the `*.in` format for now. Re-evaluate when we have
  more resources and resource types.
