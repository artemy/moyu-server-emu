import glob
import os

ASCIIDOCTOR_REVEALJS = 'asciidoctor-revealjs'
ASCIIDOCTOR_REVEALJS_OPTS = '-r asciidoctor-diagram '

OUT_DIR = 'out'


def build_revealjs_slides():
    out_filename = "out/slides.html"
    command_parts = [ASCIIDOCTOR_REVEALJS,
                     ASCIIDOCTOR_REVEALJS_OPTS,
                     "-a imagesdir=../slides/includes/",
                     'slides/index.adoc',
                     "-o {}".format(out_filename)]
    command = " ".join(command_parts)
    print(command)
    os.system(command)

def main():
    build_revealjs_slides()


if __name__ == "__main__":
    main()
