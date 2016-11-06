library(ggplot2)

data = read.csv("benchmarks.csv")

data.medians = subset(data, grepl('Filtre médian', Filtre))
data.simple_medians = subset(data.medians, !grepl('optimisé', Filtre))
data.sobels = subset(data, grepl('Sobel', Filtre))
data.process = subset(data, grepl('Traitement', Filtre))
data.optimal = subset(data, grepl('Filtre médian histogramme optimisé|Sobel optimisé et seuillage', Filtre))

limits <- aes(ymax=Durée + Ecart.type, ymin=Durée - Ecart.type)

pdf("simple_medians.pdf", height=6)
ggplot(data.simple_medians, aes(x=Taille.du.filtre, y=Durée, fill=Filtre)) + xlab("Taille du filtre") + ylab("Durée (ns)") + theme(legend.position=c(.8, .2)) + geom_bar(stat="identity", position=position_dodge()) + geom_errorbar(limits, width=1)
dev.off()

pdf("medians.pdf", height=6)
ggplot(data.medians, aes(x=Taille.du.filtre, y=Durée, color=Filtre)) + xlab("Taille du filtre") + ylab("Durée (ns)") + theme(legend.position=c(.8, .3)) + geom_line() + geom_point() + geom_errorbar(limits, width=1)
dev.off()

pdf("sobels.pdf", height=5)
ggplot(data.sobels, aes(x=Filtre, fill=Filtre, y=Durée)) + ylab("Durée (ns)") + theme(legend.position=c(.8, .2)) + geom_bar(stat="identity", position="dodge") + geom_errorbar(limits, width=.5)
dev.off()

data.tmp_optimal <- data.optimal
data.tmp_optimal$partial <- "Partiel"

data.tmp_process <- data.process
data.tmp_process$partial <- "Final"

data.tmp <- rbind(data.tmp_optimal, data.tmp_process)

pdf("process.pdf", height=4)
ggplot(data.tmp, aes(x=Taille.du.filtre, fill=Filtre, y=Durée)) + xlab("Taille du filtre médian") + ylab("Durée (ns)") + theme(legend.position=c(.8, .2)) + geom_bar(stat="identity", position="stack") + facet_wrap( ~ partial)
dev.off()
